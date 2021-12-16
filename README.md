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

NURBS can represent multiple Bézier curve segments in a single curve. The
example below shows an outline of the Egyptian "reed leaf" hieroglyph
(Gardiner sign M17). This curve is constructed from 2 line segments and 4
cubic Bézier curves, all of which can be represented as a single closed
NURBS curve:

<img src="./diagrams/reed-leaf.svg" width="300" height="300"/>

## NURBS Curve Representation

The library uses the "Rhino" form of NURBS curves, where there are two fewer
knots than in "traditional" NURBS.

## Usage

A simple circle example demonstrating how to obtain an interpolated value at an arbitrary location on a curve.

```rust
use nalgebra::Vector2;
use capstan::{knotvec::KnotVec};
type Curve = capstan::curve::Curve<f32, Vector2<f32>>;

fn main() {

    let r = f32::sqrt(2.0) / 2.0;
    let degree = 2;
    let control_points = vec![
        Vector2::new(1.0, 0.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(0.0, 1.0),
        Vector2::new(-1.0, 1.0),
        Vector2::new(-1.0, 0.0),
        Vector2::new(-1.0, -1.0),
        Vector2::new(0.0, -1.0),
        Vector2::new(1.0, -1.0),
        Vector2::new(1.0, 0.0),
    ];
    let weights = vec![1.0, r, 1.0, r, 1.0, r, 1.0, r, 1.0];
    let knots = KnotVec::new(vec![
        0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
    ])
    .unwrap();

    let circle = Curve::new(degree, control_points, weights, knots).unwrap();

    let u = 0.5_f32;

    // The de_boor method is used to obtainin an interpolated point
    println!("interpolation paramater value {}:{}", u, circle.de_boor(u))
}
```

## Examples

The examples folder includes the above as well as the more comprehensive svg_example.rs example that outputs a number of svg files. The svg crate is used for this but is only required for demonstration purposes.

To run the svg_example:
$ cargo run --example svg_example

This will output a number of .svg files into the working directory