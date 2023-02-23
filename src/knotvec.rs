use core::ops::Index;
use is_sorted::IsSorted;

use crate::algebra::ScalarT;

/// Vector of knots in non-decreasing order.
///
/// Knot values exist in the parameter space of a NURBS curve. They partition
/// the total 1D parameter range into regions over which the interpolating
/// polynomials of the NURBS curve are active. Thus, in combination with the
/// degree of a NURBS curve, they define the non-uniform B-spline basis
/// functions.
#[derive(Clone, Debug, PartialEq)]
pub struct KnotVec<N: ScalarT> {
    knots: Vec<N>,
}

impl<N: ScalarT> KnotVec<N> {
    /// Creates a new knot vector if possible.
    ///
    /// A new knot vector must satisfy the following criteria:
    /// * it must contain >= 2 elements
    /// * it must be sorted (non-decreasing)
    /// * it must represent a non-zero span (the last knot cannot be equal to
    ///   the first)
    ///
    /// # Parameters
    ///
    /// * `knots` - a vector of knot values in non-decreasing order
    ///
    /// # Example
    ///
    /// ```
    /// # use capstan::KnotVec;
    /// let knots = KnotVec::new(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0]).unwrap();
    /// ```
    pub fn new(knots: Vec<N>) -> Option<Self> {
        if knots.len() >= 2
            && IsSorted::is_sorted(&mut knots.iter())
            && &knots[0] != knots.last().unwrap()
        {
            Some(KnotVec { knots })
        } else {
            None
        }
    }

    /// Returns the number of knots in the knot vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use capstan::KnotVec;
    /// let knots = KnotVec::new(vec![0.0, 0.0, 1.0, 1.0]).unwrap();
    /// assert_eq!(knots.len(), 4);
    /// ```
    pub fn len(&self) -> usize {
        self.knots.len()
    }

    /// Checks if a knot vector is clamped.
    ///
    /// A knot vector is clamped if the first knot value is repeated
    /// `degree + 1` times at the start of the knot vector (ie. its
    /// multiplicity is `degree + 1`), and if the last knot is repeated
    /// `degree + 1` times at the end of the knot vector.
    ///
    /// # Parameters
    ///
    /// * `degree` - degree of the NURBS curve
    pub fn is_clamped(&self, degree: usize) -> bool {
        if self.knots.len() < 2 * (degree + 1) {
            false
        } else {
            // check the value of the start knots
            let start_knot = self.knots[0];
            for i_knot in &self.knots[1..degree] {
                if *i_knot != start_knot {
                    return false;
                }
            }

            // check the value of the end knots
            let end_knot = self.knots.last().unwrap();
            for e_knot in &self.knots[self.knots.len() - degree - 1..self.knots.len() - 1] {
                if e_knot != end_knot {
                    return false;
                }
            }

            // everything passed
            true
        }
    }

    /// Checks if the knot vector is empty (always returns `false`).
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Returns the minimum parameter value contained in this knot vector.
    pub fn min_u(&self) -> N {
        self.knots[0]
    }

    /// Returns the maximum parameter value contained in this knot vector.
    pub fn max_u(&self) -> N {
        *self
            .knots
            .last()
            .expect("last() should always succeed, because there should be >=2 knots")
    }

    /// Clamp a parameter value to the allowed range of the parameter.
    ///
    /// # Parameters
    ///
    /// * `u` - the parameter value to clamp in the range `min_u <= u <= max_u`
    pub fn clamp(&self, u: N) -> N {
        if u < self.min_u() {
            self.min_u()
        } else if u > self.max_u() {
            self.max_u()
        } else {
            u
        }
    }

    /// Finds the index of the span inside the knot vector which contains the
    /// parameter value `u`.
    ///
    /// Each pair of knots in the knot vector defines a span. Spans are
    /// zero length for knots with a multiplicity > 1. Given a parameter
    /// value, `u`, this function returns the index of the knot span which
    /// contains `u`.
    ///
    /// The knot span index, `i = knots.find_span(u)`, which contains u,
    /// satisfies the relationship that:
    ///
    /// ```text
    /// knots[i] <= u < knots[i+1], when u < knots.max_u()
    /// knots[i] < u == knots[i+1], when u == knots.max_u()
    /// ```
    ///
    /// # Parameters
    ///
    /// * `u` - the parameter value for which to find the span
    ///
    /// # Examples
    ///
    /// ```
    /// # use capstan::KnotVec;
    /// let knots = KnotVec::new(vec![0.0, 0.0, 0.5, 1.0, 1.0]).unwrap();
    /// assert_eq!(knots.find_span(0.0), 1);
    /// assert_eq!(knots.find_span(0.6), 2);
    /// assert_eq!(knots.find_span(1.0), 2); // note u=knots.max_u() is a bit special
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the parameter `u` is outside the allowed range of the knot
    /// vector.
    pub fn find_span(&self, u: N) -> usize {
        debug_assert!(
            u >= self.min_u(),
            "parameter u={:?} is below the required range {:?} <= u <= {:?}",
            u,
            self.min_u(),
            self.max_u()
        );
        debug_assert!(
            u <= self.max_u(),
            "parameter u={:?} is above the required range {:?} <= u <= {:?}",
            u,
            self.min_u(),
            self.max_u()
        );

        if u == self.max_u() {
            // if we have the maximum u value then handle that as a special case;
            // look backward through the knots until we find one which is less
            // than the maximum u value
            self.knots
                .iter()
                .enumerate()
                .rev()
                .find(|&item| item.1 < &u)
                .unwrap()
                .0
        } else {
            // perform a binary search to find the correct knot span
            let mut low: usize = 0;
            let mut high: usize = self.len() - 1;
            let mut mid: usize = (low + high) / 2;

            while u < self.knots[mid] || u >= self.knots[mid + 1] {
                if u < self.knots[mid] {
                    high = mid;
                } else {
                    low = mid;
                }
                mid = (low + high) / 2;
            }

            mid
        }
    }
}

impl<N: ScalarT> Index<usize> for KnotVec<N> {
    type Output = N;

    fn index(&self, i: usize) -> &Self::Output {
        &self.knots[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    /// Test creating a new knot vector.
    #[test]
    fn new() {
        let knots = KnotVec::new(vec![0.0, 0.0, 0.5, 1.0, 1.0]).unwrap();
        assert_eq!(knots.len(), 5);
        assert_eq!(knots.is_empty(), false);
        assert_eq!(knots[0], 0.0);
        assert_eq!(knots[1], 0.0);
        assert_eq!(knots[2], 0.5);
        assert_eq!(knots[3], 1.0);
        assert_eq!(knots[4], 1.0);
        assert_eq!(knots.min_u(), 0.0);
        assert_eq!(knots.max_u(), 1.0);
    }

    /// Test the is_clamped method.
    #[test]
    fn is_clamped() {
        let knots1 = KnotVec::new(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0]).unwrap();
        assert!(knots1.is_clamped(2));
        assert!(!knots1.is_clamped(3));

        let knots2 = KnotVec::new(vec![0.0, 0.0, 0.0, 1.0, 1.0]).unwrap();
        assert!(knots2.is_clamped(1));
        assert!(!knots2.is_clamped(2));
        assert!(!knots2.is_clamped(100));
    }

    /// Test clamping the paramter.
    #[test]
    fn clamp() {
        let knots = KnotVec::new(vec![0.0, 0.0, 1.0, 1.0]).unwrap();
        assert_eq!(knots.clamp(-1.0), 0.0);
        assert_eq!(knots.clamp(0.5), 0.5);
        assert_eq!(knots.clamp(1.2), 1.0);
    }

    /// A knot vector must always have at least two knots, so that it has a
    /// span.
    #[test]
    fn less_than_two_knots() {
        assert_eq!(KnotVec::new(vec![0.0]), None);
    }

    /// Knots must be in non-decreasing order.
    #[test]
    fn badly_ordered_knots() {
        assert_eq!(KnotVec::new(vec![1.0, 0.0]), None);
    }

    /// Knots cannot be degenerate; they must span some non-zero range.
    #[test]
    fn degenerate_knots() {
        assert_eq!(KnotVec::new(vec![0.0, 0.0, 0.0]), None);
    }

    /// Test finding the knot span that contains a parameter value.
    #[test]
    fn find_span() {
        let knots = KnotVec::new(vec![0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 5.0, 5.0]).unwrap();
        assert_eq!(knots.find_span(0.0), 1);
        assert_eq!(knots.find_span(3.001), 4);
        assert_eq!(knots.find_span(4.0), 6);
        assert_eq!(knots.find_span(5.0), 6);
    }

    #[test]
    #[should_panic(expected = "parameter u=0.5 is below the required range 1.0 <= u <= 5.0")]
    fn find_span_panic_when_u_is_too_low() {
        let knots = KnotVec::new(vec![1.0, 1.0, 1.0, 5.0, 5.0, 5.0]).unwrap();
        knots.find_span(0.5);
    }

    #[test]
    #[should_panic(expected = "parameter u=5.5 is above the required range 1.0 <= u <= 5.0")]
    fn find_span_panic_when_u_is_too_high() {
        let knots = KnotVec::new(vec![1.0, 1.0, 1.0, 5.0, 5.0, 5.0]).unwrap();
        knots.find_span(5.5);
    }

    prop_compose! {
        fn arb_knotvec(min_len: usize)
                      (len in min_len..128)
                      (mut ks in proptest::collection::vec(any::<f32>(), len)) -> KnotVec<f32>
        {
            // make sure the knot vector is sorted
            ks.sort_by(|a, b| a.partial_cmp(b).unwrap());

            // make sure the knot vector is non-degenerate (it must contain more
            // than one unique value)
            if ks.last().unwrap() == &ks[0] {
                // if it's degenerate, add an extra, non-degenerate knot on the end
                ks.push(ks.last().unwrap() + 1.0);
            }

            KnotVec::new(ks).unwrap()
        }
    }

    prop_compose! {
        fn arb_knotvec_and_param()
                                (knotvec in arb_knotvec(2))
                                (u in knotvec.min_u()..knotvec.max_u(), knotvec in Just(knotvec)) -> (f32, KnotVec<f32>)
        {
            (u, knotvec)
        }
    }

    proptest! {
        /// For an arbitrary knot vector and parameter value, the span index
        /// found for the parameter must actually contain that parameter value.
        #[test]
        fn knot_span_contains_knot((u, knotvec) in arb_knotvec_and_param()) {
            let i: usize = knotvec.find_span(u);
            assert!(knotvec[i] <= u);
            if u < knotvec.max_u() {
                assert!(knotvec[i+1] >= u);
            } else {
                assert!(knotvec[i+1] > u);
            }
        }
    }
}
