# Capstan

![GitHub Rust CI](https://github.com/lancelet/capstan/workflows/Rust/badge.svg)
[![Codecov.io](https://codecov.io/gh/lancelet/capstan/branch/main/graph/badge.svg)](https://codecov.io/gh/lancelet/capstan)
[![Crates.io](https://img.shields.io/crates/v/capstan.svg)](https://crates.io/crates/capstan)
[![Docs.rs](https://docs.rs/capstan/badge.svg)](https://docs.rs/capstan)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

NURBS utilities in Rust.

## NURBS Curve Evaluation

Currently, only NURBS curve evaluation is complete. The evaluation uses a
naive version of the de Boor algorithm. With this, it's possible to evaluate
the 3D coordinates of a NURBS curve at any parameter value.

NURBS can represent conics with floating-point precision. This image shows a
tesselated NURBS circle on the left and an SVG circle on the right:

<img src="./diagrams/circle.svg" width="600" height="300"/>

NURBS are a generalization of Bézier curves, so they can exactly represent any
order of Bézier curve. The image below shows an SVG cubic Bézier with a loop on
the right and a tesselated NURBS representation on the left:

<img src="./diagrams/cubic-bezier.svg" width="600" height="300"/>

## NURBS Curve Representation

The library uses the "Rhino" form of NURBS curves, where there are two fewer
knots than in "traditional" NURBS.