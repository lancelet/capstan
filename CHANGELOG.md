# Changelog

Please track all notable changes in this file. This format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.0.0).

## [Unreleased]

## [0.0.4]

### Added

- Mention of example in README.md

### Changed

- Bumped rust edition to 2021
- Upgraded the Nalbegra dependency to 0.29
- Implemented VectorT for OVector instead of VectorN as VectorN is now deprecated.
- Moved main.rs to the examples folder
- Moved the SVG dependency to dev-dependencies so that it won't be included by default in release builds
- Edited .gitignore to ignore root level .svg artifacts generated from the example

## [0.0.3]

### Added

- `KnotVec.is_clamped` function.
- "Reed leaf hieroglyph" example of a curve with multiple Bézier segments.

### Changed

- Changed the representation to use full-multiplicity knots instead of the
  "Rhino" style.
- `KnotVec` is now passed to `Curve` on creation, instead of an arbitrary
  `Vec`.

## [0.0.2]

### Added

- `ScalarT` and `VectorT` traits to represent the operations required of
  scalars and vectors used by NURBS curves and surfaces.
- `KnotVec` type to represent knot vectors in NURBS curves. This will later be
  shared with NURBS surfaces.
- `codecov.io` test coverage using `grcov`.
- README badges for `codecov.io`, `crates.io`, `docs.rs` and the LICENSE.

### Changed

- `Curve` type is now parameterised by scalar and vector types.
- Examples in `main` use `nalgebra::Vector2` instead of 3D points.

## [0.0.1]

### Added

- Initial project setup.
- NURBS Curve representation and evaluation using the de Boors algorithm.
- Plot some examples for the README.

[unreleased]: https://github.com/lancelet/capstan/compare/v0.0.3...HEAD
[0.0.3]: https://github.com/lancelet/capstan/releases/tag/v0.0.3
[0.0.2]: https://github.com/lancelet/capstan/releases/tag/v0.0.2
[0.0.1]: https://github.com/lancelet/capstan/releases/tag/v0.0.1