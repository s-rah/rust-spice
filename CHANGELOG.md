# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## TODO

## [Unreleased]

## [0.6.16 - 0.6.17] - 2021-06-08

### Changed

+ strings in input are now `&str`
+ strings in output stay `String`

Quick recall that functions in C that ouput string always ask for allocation
size. In this lib, neat functions default to `spice::MAX_LEN_OUT`, and you are
free to call the raw version of the fonction to get the hand of the signature
with the size argument.

## [0.6.14 - 0.6.15] - 2021-06-07

### Added

+ dskobj
+ Cell (only integer for now)
+ bodc2n
+ bodn2c

## [0.6.11 - 0.6.13] - 2021-06-06

### Added

+ ktotal
+ kdata (default length 256)

### Fixed

+ string allocation for output parameters

## [0.6.7 - 0.6.10] - 2021-06-05

### Added

+ dskz02
+ dskp02
+ dskv02
+ illumf

## [0.6.6] - 2021-06-04

### Added

+ kclear

### Fixed

+ clippy warnings

## [0.6.5] - 2021-05-28

### Added

+ dla & dsk descr type alias

## [0.6.3 - 0.6.4] - 2021-05-24

### Added

+ dascls
+ dasopr
+ dlabfs
+ dskgd
+ dskn02
+ dskx02
+ latrec
+ recrad
+ vsep

## [0.6.2] - 2021-05-17

### Fixed

+ some links

## [0.6.1] - 2021-05-17

### Added

+ documentation

## [0.6.0] - 2021-05-17

### Added

+ crate `rust-spice-derive`

### Changed

+ implement Rust idiomatic interface to CSPICE with procedural macros
+ moved `rust-spice` crate to subforlder of `rust-spice` root repo, to contain
  the main crate and the derive

### Removed

+ mods `spicetools`, `kernel`, `check_geometric_conditions`
+ examples, for now if you want example -> go check tests

## [0.5.4] - 2021-05-13

### Added

+ documentation

## [0.5.1 - 0.5.3] - 2021-05-13

Many trials to make CSPICE wrapper working on Mac and Windows also.
Ended up proposing to overwrite the build of `cspice-sys` inside
`$HOME/.cargo/config.toml`, thanks to the `links` attributes.

## [0.4.8] - 2021-05-12

### Added

+ implement occult_c

### Changed

+ improve readme
+ simplify error handling with crate `thiserror`

### Removed

+ unnecessary examples
+ unnecessary tests

## [0.4.7] - 2021-04-23

### Changed

+ using nalgebra 0.26

## [0.4.6] - 2021-04-16

### Added

+ more examples

### Changed

+ use rustool 0.3.12

## [0.4.5] - 2021-04-09

### Changed

+ improved example

## [0.4.4] - 2021-04-09

### Added

+ tests kernel
+ frames functions
+ example frames

## [0.4.3] - 2021-04-05

### Added

+ error managment
+ examples
+ tests

### Changed

+ System getters not mutable

### Removed

+ module toolbox is now external to this crate
+ static list of loaded kernels, let user deal with asynchronous problem if they
  want to do it, this crate ensures the type Kernel loads and unloads correctly

## [0.4.2] - 2021-04-01

### Added

+ documentation guide online

## [0.4.1] - 2021-04-01

### Added

+ documentation and test coverages in badges

## [0.4.0] - 2021-04-01

### Changed

+ this crate is not anymore creating the binding, it uses cspice-rust, to focus
  more on the Rust layer.

## [0.3.5] - 2021-03-31

### Changed

+ spicetools toolbox is now 100% Rust -> safer and simplier build script

## [0.3.4] - 2021-03-29

### Added

+ example
+ documentation

## [0.3.3] - 2021-03-29

### Added

+ Rust layer for the struct System
+ documentation
+ test on 100% of C spicetools

## [0.3.2] - 2021-03-29

+ moved cspice to OUT_DIR, I don't like it because wget procs on every build of
  every single different target, didn't find a workaround yet

## [0.3.1] - 2021-03-29

### Added

+ README comment on objectives

## [0.3.0] - 2021-03-29

### Removed

+ dependency spice-sys

### Modified

+ spice-sys script to get cscpice
+ README

## [0.2.1] - 2021-03-29

+ tried to incorporate spice-sys to use its build script to get cspice but the
  OUT_DIR environment variable is not configurable for dependency so the cspice
  library was hidden in target/ and it was not easy to link it automatically
+ cspice was being downloaded every time the build is launch... and multiple
  time for the different targets, so it was annoying and had to move to v0.3.0

## [0.2.0] - 2021-03-28

### Added

+ cspice submodule (fork of official code)
+ some lib test

### Changed

+ spicetools submodule (refer to v0.2.0 in spicetools CHANGELOG)
+ cspice lib name
+ build script to correctly link with spice and spicetools functions

## [0.1.0] - 2021-03-27

+ version not working, I was trying to understand concepts of wrapping and
  binding with bindgen.

### Added

+ Initial commit
