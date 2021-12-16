//! A simple circle example demonstrating how to obtain an 
//! interpolated value at an arbitrary location on a curve.

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
    println!("interpolation paramater value {}:{}", u, circle.de_boor(u))
}