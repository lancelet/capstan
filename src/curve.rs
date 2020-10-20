use crate::algebra::{ScalarT, VectorT};
use crate::knotvec::KnotVec;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, CurveError>;

/// NURBS curve.
///
/// Non-Uniform Rational B-Spline.
#[derive(PartialEq, Debug)]
pub struct Curve<N, V>
where
    N: ScalarT,
    V: VectorT<Field = N>,
{
    degree: usize,
    control_points: Vec<V>,
    weights: Vec<N>,
    knots: KnotVec<N>,
}

impl<N, V> Curve<N, V>
where
    N: ScalarT,
    V: VectorT<Field = N>,
{
    /// Creates a new NURBS Curve.
    ///
    /// The following basic properties must be satisfied for a NURBS curve:
    /// * `degree` > 0
    /// * `control_points.len() > degree`
    /// * `weights.len() == control_points.len()`
    /// * `knots.len() == degree + control_points.len() + 1`
    /// * `knots.is_clamped()`
    ///
    /// The NURBS curves represented here are clamped (ie. they must have a
    /// knot multiplicity at either end equal to the degree plus one).
    /// Un-clamped curves can be converted to clamped ones via knot insertion.
    ///
    /// Parameters:
    ///
    /// * `degree` - polynomial degree of the NURBS curve
    /// * `control_points` - vector of control points
    /// * `weights` - vector of weights (must be the same length as
    ///               `control_points`)
    /// * `knots` - knot vector (must have `degree + control_points.len() + 1`
    ///             elements)
    pub fn new(
        degree: usize,
        control_points: Vec<V>,
        weights: Vec<N>,
        knots: KnotVec<N>,
    ) -> Result<Self> {
        if degree == 0 {
            Err(CurveError::InvalidDegree)
        } else if control_points.len() <= degree {
            Err(CurveError::InsufficientControlPoints {
                degree,
                number_supplied: control_points.len(),
            })
        } else if weights.len() != control_points.len() {
            Err(CurveError::MismatchedWeightsAndControlPoints)
        } else if knots.len() != degree + control_points.len() + 1 {
            Err(CurveError::InvalidKnotCount {
                required_knot_len: degree + control_points.len() + 1,
                receieved_knot_len: knots.len(),
            })
        } else if !knots.is_clamped(degree) {
            Err(CurveError::KnotVectorNotClamped)
        } else {
            Ok(Curve {
                degree,
                control_points,
                weights,
                knots,
            })
        }
    }

    /// Interpolates the curve at a parameter value.
    ///
    /// This method uses the
    /// [de Boor algorithm](https://en.wikipedia.org/wiki/De_Boor%27s_algorithm)
    /// to evaluate the NURBS curve at a given parameter value `u`. The de Boor
    /// algorithm is a good choice for efficiently evaluating a NURBS curve and
    /// is numerically stable.
    ///
    /// The parameter `u` is clamped to the allowed range of the parameter
    /// space of the curve (which is the range from `self.knots().min_u()` to
    /// `self.knots().max_u()` inclusive).
    ///
    /// # Parameters
    ///
    /// * `u` - the parameter value at which to evaluate the NURBS curve
    pub fn de_boor(&self, u: N) -> V {
        // clamp u and find the knot span containing u
        let uu = self.knots.clamp(u);
        let k = self.knots.find_span(uu);

        // populate initial triangular column
        let mut d = Vec::<V>::with_capacity(self.degree + 1); // homogeneous points
        let mut dw = Vec::<N>::with_capacity(self.degree + 1); // weights
        for j in 0..self.degree + 1 {
            let i: usize = j + k - self.degree;

            // multiply the control points by the corresponding weight to
            // convert from Cartesian to homogeneous coordinates
            d.push(self.control_points[i].clone() * self.weights[i]);
            dw.push(self.weights[i]);
        }

        // make extra-sure we allocated enough capacity
        debug_assert!(d.len() <= self.degree + 1);
        debug_assert!(dw.len() <= self.degree + 1);

        // main de Boor algorithm
        for r in 1..self.degree + 1 {
            for j in (r..self.degree + 1).rev() {
                let kp = self.knots[j + k - self.degree];
                let alpha = (uu - kp) / (self.knots[1 + j + k - r] - kp);
                let nalpha = N::one() - alpha;
                d[j] = d[j - 1].clone() * nalpha + d[j].clone() * alpha;
                dw[j] = dw[j - 1] * nalpha + dw[j] * alpha;
            }
        }

        // convert final coordinate from homogeneous to Cartesian coords
        d[self.degree].clone() * (N::one() / dw[self.degree])
    }

    /// Returns the vector of control points.
    pub fn control_points(&self) -> &Vec<V> {
        &self.control_points
    }

    /// Returns the knot vector.
    pub fn knots(&self) -> &KnotVec<N> {
        &self.knots
    }

    /// Scale the curve by a uniform amount about the origin.
    ///
    /// NOTE: This method will probably be replaced by a more general
    ///       transformation method in the future.
    pub fn uniform_scale(&mut self, scale_factor: N) {
        for cp in &mut self.control_points {
            *cp = cp.clone() * scale_factor;
        }
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum CurveError {
    #[error("invalid degree; must satisfy degree > 0")]
    InvalidDegree,

    #[error("N={} control points were supplied; at least {} are required \
             for a degree {} curve",
            .number_supplied,
            .degree,
            .degree - 1)]
    InsufficientControlPoints {
        degree: usize,
        number_supplied: usize,
    },

    #[error("number of weights and control points must be identical")]
    MismatchedWeightsAndControlPoints,

    #[error("expected {} knot values, but received {}",
            .required_knot_len,
            .receieved_knot_len)]
    InvalidKnotCount {
        required_knot_len: usize,
        receieved_knot_len: usize,
    },

    #[error("knot vector was not clamped")]
    KnotVectorNotClamped,
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use nalgebra::Vector2;

    /// Test Curve
    type TC = Curve<f32, Vector2<f32>>;

    /// The degree of a NURBS curve must be >= 0.
    #[test]
    fn invalid_degree() {
        let result = TC::new(0, vec![], vec![], KnotVec::new(vec![0.0, 1.0]).unwrap());
        assert_eq!(result, Err(CurveError::InvalidDegree));
    }

    /// There must be at least degree + 1 control points.
    #[test]
    fn insufficient_control_points() {
        let result = TC::new(
            1,
            vec![Vector2::new(0.0, 0.0)],
            vec![1.0],
            KnotVec::new(vec![0.0, 0.0, 1.0, 1.0]).unwrap(),
        );
        assert_eq!(
            result,
            Err(CurveError::InsufficientControlPoints {
                degree: 1,
                number_supplied: 1
            })
        )
    }

    /// The number of control points and weights must be identical.
    #[test]
    fn weights_and_cps_lengths_must_be_equal() {
        let result = TC::new(
            1,
            vec![Vector2::new(0.0, 0.0), Vector2::new(42.0, 56.0)],
            vec![1.0],
            KnotVec::new(vec![0.0, 1.0]).unwrap(),
        );
        assert_eq!(result, Err(CurveError::MismatchedWeightsAndControlPoints));
    }

    /// The correct number of knot values must be supplied.
    #[test]
    fn invalid_knot_count() {
        let result = TC::new(
            1,
            vec![Vector2::new(0.0, 0.0), Vector2::new(42.0, 56.0)],
            vec![1.0, 1.0],
            KnotVec::new(vec![0.0, 1.0]).unwrap(),
        );
        assert_eq!(
            result,
            Err(CurveError::InvalidKnotCount {
                required_knot_len: 4,
                receieved_knot_len: 2
            })
        );
    }

    /// Test that we detect a non-clamped knot vector.
    #[test]
    fn knot_vector_not_clamped() {
        let result = TC::new(
            2,
            vec![
                Vector2::new(0.0, 0.0),
                Vector2::new(1.0, 2.0),
                Vector2::new(3.0, 4.0),
            ],
            vec![1.0, 1.0, 1.0],
            KnotVec::new(vec![0.0, 0.0, 0.5, 0.5, 0.9, 1.0]).unwrap(),
        );
        assert_eq!(result, Err(CurveError::KnotVectorNotClamped));
    }

    /// Creating a new NURBS curve successfully.
    #[test]
    fn new() {
        let nurbs = TC::new(
            1,
            vec![Vector2::new(0.0, 0.0), Vector2::new(42.0, 56.0)],
            vec![1.0, 1.0],
            KnotVec::new(vec![0.0, 0.0, 1.0, 1.0]).unwrap(),
        )
        .unwrap();
        assert_eq!(nurbs.knots().min_u(), 0.0);
        assert_eq!(nurbs.knots().max_u(), 1.0);
        assert_eq!(
            nurbs.control_points(),
            &vec![Vector2::new(0.0, 0.0), Vector2::new(42.0, 56.0)]
        );
    }

    /// Uniformly scaling a NURBS curve.
    #[test]
    fn uniform_scale() {
        let mut nurbs = TC::new(
            1,
            vec![Vector2::new(0.0, 0.0), Vector2::new(42.0, 56.0)],
            vec![1.0, 1.0],
            KnotVec::new(vec![0.0, 0.0, 1.0, 1.0]).unwrap(),
        )
        .unwrap();
        nurbs.uniform_scale(2.0);

        let expected = TC::new(
            1,
            vec![Vector2::new(0.0, 0.0), Vector2::new(2.0 * 42.0, 2.0 * 56.0)],
            vec![1.0, 1.0],
            KnotVec::new(vec![0.0, 0.0, 1.0, 1.0]).unwrap(),
        )
        .unwrap();

        assert_eq!(nurbs, expected);
    }

    /// Test de Boor evalutaion on a non-rational, uniform Bezier.
    #[test]
    fn de_boor_non_rational_uniform_bezier() {
        let test_curve = TC::new(
            3,
            vec![
                Vector2::new(-10.0, 10.0),
                Vector2::new(10.0, 10.0),
                Vector2::new(-10.0, -10.0),
                Vector2::new(10.0, -10.0),
            ],
            vec![1.0, 1.0, 1.0, 1.0],
            KnotVec::new(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0]).unwrap(),
        )
        .unwrap();

        // tests for in-range parameter
        assert_relative_eq!(Vector2::new(-10.0, 10.0), test_curve.de_boor(0.0));
        assert_relative_eq!(Vector2::new(-2.16, 7.92), test_curve.de_boor(0.2));
        assert_relative_eq!(Vector2::new(0.0, 0.0), test_curve.de_boor(0.5));
        assert_relative_eq!(Vector2::new(10.0, -10.0), test_curve.de_boor(1.0));

        // tests with parameter out-of-range (clipped to parameter range)
        assert_relative_eq!(Vector2::new(-10.0, 10.0), test_curve.de_boor(-1.0));
        assert_relative_eq!(Vector2::new(10.0, -10.0), test_curve.de_boor(2.0));
    }
}
