use alga::general::RealField;
use nalgebra::geometry::Point3;
use nalgebra::geometry::Point4;
use nalgebra::Scalar;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, CurveError>;

#[derive(PartialEq, Debug)]
pub struct Curve<N: Scalar> {
    degree: usize,
    control_points: Vec<Point4<N>>,
    normalized_control_points: Vec<Point4<N>>,
    knots: Vec<N>,
}

impl<N: Scalar + PartialOrd + RealField> Curve<N> {
    pub fn new(degree: usize, control_points: Vec<Point4<N>>, knots: Vec<N>) -> Result<Self> {
        if degree == 0 {
            Err(CurveError::InvalidDegree)
        } else {
            let required_knot_len = degree + control_points.len() - 1;
            if control_points.len() <= degree {
                Err(CurveError::InsufficientControlPoints {
                    degree,
                    number_supplied: control_points.len() as u16,
                })
            } else if knots.len() != required_knot_len {
                Err(CurveError::InvalidKnotCount {
                    number_received: knots.len(),
                    number_expected: required_knot_len,
                })
            } else if !Self::knots_ordered(&knots) {
                Err(CurveError::InvalidKnotOrder)
            } else {
                let mut normalized_control_points = Vec::new();
                for cp in &control_points {
                    normalized_control_points.push(Point4::new(
                        cp.coords.x * cp.coords.w,
                        cp.coords.y * cp.coords.w,
                        cp.coords.z * cp.coords.w,
                        cp.coords.w,
                    ))
                }
                Ok(Curve {
                    degree,
                    control_points,
                    normalized_control_points,
                    knots,
                })
            }
        }
    }

    pub fn de_boor(self: &Self, u: N) -> Point3<N> {
        let p = self.degree;

        // Construct a traditional knot vector with extra terminal knots
        let mut ks = Vec::with_capacity(self.knots.len() + 2);
        ks.push(self.knots[0].clone());
        ks.append(&mut self.knots.clone());
        ks.push(self.knots.last().unwrap().clone());

        // Find minimum and maximum parameter values
        let min_param = &ks[0];
        let max_param = ks.last().unwrap();

        // clamp u to the allowed range
        let mut uu = u;
        if &uu < min_param {
            uu = min_param.clone();
        }
        if &uu > max_param {
            uu = max_param.clone();
        }

        // find the index of the knot interval that contains u
        // needs some special handling at the end of the parameter range
        let mut k;
        if &uu < max_param {
            k = 0;
            loop {
                if ks[k] <= uu && ks[k + 1] > uu {
                    break;
                } else {
                    k += 1;
                }
            }
        } else {
            k = ks.len() - 1;
            while ks[k - 1] == ks[k] {
                k -= 1;
            }
            k -= 1;
        }

        // populate the initial d values
        let mut d = Vec::new();
        for j in 0..(p + 1) {
            d.push(self.normalized_control_points[j + k - p].clone().coords);
        }

        // main de Boor algorithm
        for r in 1..(p + 1) {
            for j in (r..p + 1).rev() {
                let alpha = (uu.clone() - ks[j + k - p].clone())
                    / (ks[j + 1 + k - r].clone() - ks[j + k - p].clone());
                d[j] = d[j - 1] * (N::one() - alpha) + d[j] * alpha;
            }
        }

        let p4 = d[p];
        Point3::new(p4.x / p4.w, p4.y / p4.w, p4.z / p4.w)
    }

    pub fn control_points(self: &Self) -> &Vec<Point4<N>> {
        &self.control_points
    }

    pub fn min_u(self: &Self) -> &N {
        &self.knots[0]
    }

    pub fn max_u(self: &Self) -> &N {
        &self.knots.last().unwrap()
    }

    pub fn uniform_scale(self: &mut Self, scale_factor: N) {
        for cp in &mut self.control_points {
            cp.coords.x *= scale_factor;
            cp.coords.y *= scale_factor;
            cp.coords.z *= scale_factor
        }
        for cp in &mut self.normalized_control_points {
            cp.coords.x *= scale_factor;
            cp.coords.y *= scale_factor;
            cp.coords.z *= scale_factor
        }
    }

    fn knots_ordered(knots: &[N]) -> bool {
        let mut current_knot = &knots[0];
        for knot in &knots[1..] {
            if current_knot <= knot {
                current_knot = knot;
            } else {
                return false;
            }
        }
        return true;
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum CurveError {
    #[error("invalid degree; must be > 0")]
    InvalidDegree,

    #[error("insufficient control points were supplied (N={}) for a curve of \
             degree {}; at least {} are required",
            .number_supplied,
            .degree,
            .degree - 1)]
    InsufficientControlPoints { degree: usize, number_supplied: u16 },

    #[error("expected {} knot values, but received {}",
            .number_expected,
            .number_received)]
    InvalidKnotCount {
        number_received: usize,
        number_expected: usize,
    },

    #[error("knots were not supplied in nondecreasing order")]
    InvalidKnotOrder,
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn invalid_degree() {
        let result = Curve::<f64>::new(0, vec![], vec![]);
        assert_eq!(result, Err(CurveError::InvalidDegree));
    }

    #[test]
    fn insufficient_control_points() {
        let result = Curve::<f64>::new(
            3,
            vec![
                Point4::new(-10.0, 10.0, 0.0, 1.0),
                Point4::new(10.0, 10.0, 0.0, 1.0),
            ],
            vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        );
        assert_eq!(
            result,
            Err(CurveError::InsufficientControlPoints {
                degree: 3,
                number_supplied: 2
            })
        );
    }

    #[test]
    fn invalid_knot_count() {
        let result = Curve::<f64>::new(
            3,
            vec![
                Point4::new(-10.0, 10.0, 0.0, 1.0),
                Point4::new(10.0, 10.0, 0.0, 1.0),
                Point4::new(-10.0, -10.0, 0.0, 1.0),
                Point4::new(10.0, -10.0, 0.0, 1.0),
            ],
            vec![0.0, 0.0, 1.0, 1.0],
        );
        assert_eq!(
            result,
            Err(CurveError::InvalidKnotCount {
                number_received: 4,
                number_expected: 6
            })
        );
    }

    #[test]
    fn invalid_knot_order() {
        let result = Curve::<f64>::new(
            3,
            vec![
                Point4::new(-10.0, 10.0, 0.0, 1.0),
                Point4::new(10.0, 10.0, 0.0, 1.0),
                Point4::new(-10.0, -10.0, 0.0, 1.0),
                Point4::new(10.0, -10.0, 0.0, 1.0),
            ],
            vec![1.0, 1.0, 1.0, 0.0, 0.0, 0.0],
        );
        assert_eq!(result, Err(CurveError::InvalidKnotOrder));
    }

    #[test]
    fn de_boor_simple() {
        let test_curve = Curve::<f64>::new(
            3,
            vec![
                Point4::new(-10.0, 10.0, 0.0, 1.0),
                Point4::new(10.0, 10.0, 0.0, 1.0),
                Point4::new(-10.0, -10.0, 0.0, 1.0),
                Point4::new(10.0, -10.0, 0.0, 1.0),
            ],
            vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        )
        .unwrap();

        // tests for in-range parameter
        assert_relative_eq!(Point3::new(-10.0, 10.0, 0.0), test_curve.de_boor(0.0));
        assert_relative_eq!(Point3::new(-2.16, 7.92, 0.0), test_curve.de_boor(0.2));
        assert_relative_eq!(Point3::new(0.0, 0.0, 0.0), test_curve.de_boor(0.5));
        assert_relative_eq!(Point3::new(10.0, -10.0, 0.0), test_curve.de_boor(1.0));

        // tests with parameter out-of-range (clipped to parameter range)
        assert_relative_eq!(Point3::new(-10.0, 10.0, 0.0), test_curve.de_boor(-1.0));
        assert_relative_eq!(Point3::new(10.0, -10.0, 0.0), test_curve.de_boor(2.0));
    }
}
