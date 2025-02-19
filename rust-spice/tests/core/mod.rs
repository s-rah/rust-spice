use itertools::multizip;

#[test]
#[serial]
fn das() {
    spice::furnsh("rsc/krn/hera_study_PO_EMA_2024.tm");
    let handle = spice::dasopr("rsc/krn/g_08438mm_lgt_obj_didb_0000n00000_v002.bds");

    let (dladsc, found) = spice::dlabfs(handle);

    assert!(found);

    let dskdsc = spice::dskgd(handle, dladsc);
    let rmax = dskdsc.co3max;

    assert!(rmax > 0f64);

    spice::dascls(handle);
    spice::unload("rsc/krn/hera_study_PO_EMA_2024.tm");
}

#[test]
#[serial]
fn dskp02() {
    spice::furnsh("rsc/krn/hera_study_PO_EMA_2024.tm");

    let handle = spice::dasopr("rsc/krn/g_08438mm_lgt_obj_didb_0000n00000_v002.bds");
    let (dladsc, _) = spice::dlabfs(handle);

    let plates = spice::dskp02(handle, dladsc);

    let expected_first_plate = [1, 2, 3];
    let expected_last_plate = [1538, 849, 848];

    assert_eq!(plates.len(), 3072);

    for (component, expected_component) in
        multizip((plates.first().unwrap().iter(), expected_first_plate.iter()))
    {
        assert_eq!(component, expected_component);
    }

    for (component, expected_component) in
        multizip((plates.last().unwrap().iter(), expected_last_plate.iter()))
    {
        assert_eq!(component, expected_component);
    }

    spice::kclear();
}

#[test]
#[serial]
fn georec() {
    // Test vectors are from https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/georec_c.html
    // Based on the Clark66 spheroid
    const CLARK66_RADIUS: f64 = 6378.2064;
    const CLARK66_FLATTENING: f64 = 1.0 / 294.9787;

    // lon, lat, alt  -> x, y, z
    let test_data: [[f64; 6]; 11] = [
        [0.0000, 90.0000, -6356.5838, 0.0000, 0.0000, 0.0000],
        [0.0000, 0.0000, -6377.2063, 1.0000, 0.0000, 0.0000],
        [90.0000, 0.0000, -6377.2063, 0.0000, 1.0000, 0.0000],
        [0.0000, 90.0000, -6355.5838, 0.0000, 0.0000, 1.0000],
        [180.0000, 0.0000, -6377.2063, -1.0000, 0.0000, 0.0000],
        [-90.0000, 0.0000, -6377.2063, 0.0000, -1.0000, 0.0000],
        [0.0000, -90.0000, -6355.5838, 0.0000, 0.0000, -1.0000],
        [45.0000, 0.0000, -6376.7921, 1.0000, 1.0000, 0.0000],
        [0.0000, 88.7070, -6355.5725, 1.0000, 0.0000, 1.0000],
        [90.0000, 88.7070, -6355.5725, 0.0000, 1.0000, 1.0000],
        [45.0000, 88.1713, -6355.5612, 1.0000, 1.0000, 1.0000],
    ];

    for test in test_data.iter() {
        let rect = spice::georec(
            test[0].to_radians(),
            test[1].to_radians(),
            test[2],
            CLARK66_RADIUS,
            CLARK66_FLATTENING,
        );
        assert_relative_eq!(rect[0], test[3], epsilon = 0.0001);
        assert_relative_eq!(rect[1], test[4], epsilon = 0.0001);
        assert_relative_eq!(rect[2], test[5], epsilon = 0.0001);
    }
}

#[test]
#[serial]
fn pxform() {
    spice::furnsh("rsc/krn/hera_study_PO_EMA_2024.tm");

    let et = spice::str2et("2027-MAR-23 16:00:00");
    let matrix = spice::pxform("J2000", "ECLIPJ2000", et);

    let expected_matrix = [
        [1.0, 0.0, 0.0],
        [0.0, 0.9174820620691818, 0.3977771559319137],
        [0.0, -0.3977771559319137, 0.9174820620691818],
    ];

    for (row, expected_row) in multizip((matrix.iter(), expected_matrix.iter())) {
        for (component, expected_component) in multizip((row.iter(), expected_row.iter())) {
            assert_relative_eq!(component, expected_component, epsilon = f64::EPSILON);
        }
    }

    spice::unload("rsc/krn/hera_study_PO_EMA_2024.tm");
}

#[test]
#[serial]
fn pxfrm2() {
    spice::furnsh("rsc/krn/hera_study_PO_EMA_2024.tm");

    let etfrom = spice::str2et("2027-MAR-23 16:00:00");
    let etto = etfrom + 30.0 * tool::MINUTE;
    let matrix = spice::pxfrm2("J2000", "J2000", etfrom, etto);

    let expected_matrix = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

    for (row, expected_row) in multizip((matrix.iter(), expected_matrix.iter())) {
        for (component, expected_component) in multizip((row.iter(), expected_row.iter())) {
            assert_relative_eq!(component, expected_component, epsilon = f64::EPSILON);
        }
    }

    spice::unload("rsc/krn/hera_study_PO_EMA_2024.tm");
}

#[test]
#[serial]
fn radec() {
    spice::furnsh("rsc/krn/hera_study_PO_EMA_2024.tm");

    // Mirfak J2000 RA and DEC
    let ra = 51.080_f64.to_radians();
    let dec = 49.861_f64.to_radians();

    // Convert to Rectangular Coordinates
    let j2000_rect = spice::radrec(1.0, ra, dec);

    // Generate the position vectors to translate from J2000 to B1950
    let mat = spice::pxform("J2000", "B1950", 0.0);

    // Perform the conversion
    let b1950_rect = spice::mxv(mat, j2000_rect);

    // Translate back to RA and DEC
    let (_, ra, dec) = spice::recrad(b1950_rect);

    // Expected B1950 RA and DEC
    let ra_b1950 = 50.185_f64;
    let dec_b1950 = 49.684_f64;

    // Compare to 3 decimal places
    assert_relative_eq!(ra.to_degrees(), ra_b1950, epsilon = 0.001);
    assert_relative_eq!(dec.to_degrees(), dec_b1950, epsilon = 0.001);

    spice::unload("rsc/krn/hera_study_PO_EMA_2024.tm");
}

#[test]
#[serial]
fn spkezr() {
    spice::furnsh("rsc/krn/hera_study_PO_EMA_2024.tm");

    // an arbitrary time
    let et = spice::str2et("2021-01-06 09:36:09.1825432 TDB");

    // sun in relation to ssb
    let (sun_ssb_posvec, _sun_ssb_lt) = spice::spkezr("sun", et, "j2000", "none", "ssb");
    // earth in relation to ssb
    let (earth_ssb_posvec, _earth_ssb_lt) = spice::spkezr("earth", et, "j2000", "none", "ssb");
    // earth in relation to sun
    let (earth_sun_posvec, _earth_sun_ly) = spice::spkezr("earth", et, "j2000", "none", "sun");

    // Quick check that the (Sun relative) earth velocity vectors are the same regardless of whether we
    // calculate them indirectly from SB or directly compared to  the Sun
    assert_eq!(earth_ssb_posvec[3] - sun_ssb_posvec[3], earth_sun_posvec[3]);
    assert_eq!(earth_ssb_posvec[4] - sun_ssb_posvec[4], earth_sun_posvec[4]);
    assert_eq!(earth_ssb_posvec[5] - sun_ssb_posvec[5], earth_sun_posvec[5]);

    spice::unload("rsc/krn/hera_study_PO_EMA_2024.tm");
}

#[test]
#[serial]
fn spkpos() {
    spice::furnsh("rsc/krn/hera_study_PO_EMA_2024.tm");

    let et = spice::str2et("2027-MAR-23 16:00:00");
    let (position, light_time) = spice::spkpos("DIMORPHOS", et, "J2000", "NONE", "HERA");

    let expected_position = [18.62640405424448, 21.054373008357004, -7.136291402940499];
    let expected_light_time = 0.00009674257074746383;

    for (component, expected_component) in multizip((position.iter(), expected_position.iter())) {
        assert_relative_eq!(component, expected_component, epsilon = f64::EPSILON);
    }

    assert_relative_eq!(light_time, expected_light_time, epsilon = f64::EPSILON);

    spice::unload("rsc/krn/hera_study_PO_EMA_2024.tm");
}

#[test]
#[serial]
fn str2et() {
    spice::furnsh("rsc/krn/hera_study_PO_EMA_2024.tm");

    let et = spice::str2et("2027-MAR-23 16:00:00");

    assert_relative_eq!(et, 859089667.1856234, epsilon = f64::EPSILON);

    spice::unload("rsc/krn/hera_study_PO_EMA_2024.tm");
}

#[test]
#[serial]
fn timout() {
    spice::furnsh("rsc/krn/hera_study_PO_EMA_2024.tm");

    let et = spice::str2et("2027-MAR-23 16:00:00");

    let date = spice::timout(et, spice::TIME_FORMAT);

    assert_eq!(date, "2027-MAR-23 16:00:00");

    spice::unload("rsc/krn/hera_study_PO_EMA_2024.tm");
}

#[test]
#[serial]
fn vdot() {
    assert_eq!(spice::vdot([1.0, 2.0, 3.0], [1.0, 2.0, 3.0]), 14.0)
}

#[test]
#[serial]
fn vcrss() {
    // Examples from https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vcrss_c.html
    assert_eq!(
        spice::vcrss([0.0, 1.0, 0.0], [1.0, 0.0, 0.0]),
        [0.0, 0.0, -1.0]
    );
    assert_eq!(
        spice::vcrss([5.0, 5.0, 5.0], [-1.0, -1.0, -1.0]),
        [0.0, 0.0, 0.0]
    );
}

#[test]
#[serial]
fn vsep() {
    let ang_1 = spice::vsep([1., 0., 0.], [1., 0., 0.]);
    let ang_2 = spice::vsep([1., 0., 0.], [0., 1., 0.]);

    assert_relative_eq!(ang_1, 0.0, epsilon = f64::EPSILON);
    assert_relative_eq!(ang_2, tool::TAU / 4., epsilon = f64::EPSILON);
}

#[test]
#[serial]
fn kdata() {
    spice::furnsh("/home/greg/doc/krn/hera/mk/hera_study_PO_EMA_2024.tm");

    let count = spice::ktotal("dsk");
    assert_eq!(count, 2);

    let (file, filtyp, source, handle, found) = spice::kdata(0, "dsk");
    assert_eq!(
        file,
        "/home/greg/doc/krn/hera/dsk/g_50677mm_rad_obj_dida_0000n00000_v001.bds"
    );
    assert_eq!(filtyp, "DSK");
    assert_eq!(
        source,
        "/home/greg/doc/krn/hera/mk/hera_study_PO_EMA_2024.tm"
    );
    assert!(handle.is_positive());
    assert_eq!(found, true);

    spice::kclear();
}

#[test]
#[serial]
fn cell() {
    spice::furnsh("/home/greg/doc/krn/hera/mk/hera_study_PO_EMA_2024.tm");

    let (file, _, _, _, found) = spice::kdata(1, "dsk");
    assert!(found);

    let cell = spice::dskobj(&file);

    assert_eq!(cell.card, 1);
    assert_eq!(cell.get_data_int(0), -658031);

    assert_eq!(spice::bodc2n(cell.get_data_int(0)).0, "DIMORPHOS");

    spice::kclear();
}
