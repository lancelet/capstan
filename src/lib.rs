//! # NURBS Curve Evaluation
//!
//! The evaluation uses a naive version of the de Boor algorithm.
//!
//! With this it’s possible to evaluate the 3D coordinates of a NURBS curve at
//! any parameter value.
//!
//! ## Example
//!
//! ```
//! # use nalgebra::Vector2;
//! # use capstan::KnotVec;
//! # type Curve = capstan::Curve<f32, Vector2<f32>>;
//! let r = f32::sqrt(2.0) / 2.0;
//! let degree = 2;
//! let control_points = vec![
//!     Vector2::new(1.0, 0.0),
//!     Vector2::new(1.0, 1.0),
//!     Vector2::new(0.0, 1.0),
//!     Vector2::new(-1.0, 1.0),
//!     Vector2::new(-1.0, 0.0),
//!     Vector2::new(-1.0, -1.0),
//!     Vector2::new(0.0, -1.0),
//!     Vector2::new(1.0, -1.0),
//!     Vector2::new(1.0, 0.0),
//! ];
//! let weights = vec![1.0, r, 1.0, r, 1.0, r, 1.0, r, 1.0];
//! let knots = KnotVec::new(vec![
//!     0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
//! ])
//! .unwrap();
//!
//! let circle = Curve::new(degree, control_points, weights, knots).unwrap();
//! assert!(-1.0 == circle.de_boor(0.5)[0]);
//! ```

mod algebra;
pub use algebra::*;

mod curve;
pub use curve::*;

mod knotvec;
pub use knotvec::*;
