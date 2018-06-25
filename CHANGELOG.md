# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Added

- Add an "unproven" Cargo feature that enables the "unproven" feature of the embedded-hal dependency.

- Implement StatefulOutputPin, ToggleableOutputPin, and InputPin traits with "unproven" feature.

## [v0.2.0] - 2018-05-12

- This crate now compiles on the stable and beta channels.

- [breaking-change] implement v0.2.0 of `embedded-hal`

- [breaking-change] this crate now requires `arm-none-eabi-gcc` to be installed and available in
  `$PATH` to compile.

## [v0.1.2] - 2018-02-09

### Fixed

- `Timer::tim*` constructors so they don't disable other peripherals.

## [v0.1.1] - 2018-01-20

### Added

- Add a "rt" Cargo feature that enables the "rt" feature of the stm32f30x dependency.

## v0.1.0 - 2018-01-17

Initial release

[Unreleased]: https://github.com/japaric/stm32f30x-hal/compare/v0.2.0...HEAD
[v0.2.0]: https://github.com/japaric/stm32f30x-hal/compare/v0.1.2...v0.2.0
[v0.1.2]: https://github.com/japaric/stm32f30x-hal/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/japaric/stm32f30x-hal/compare/v0.1.0...v0.1.1
