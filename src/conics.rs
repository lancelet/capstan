use crate::algebra::ScalarT;
use crate::curve::Curve;
use alga::general::RealField;
use nalgebra::Vector2;

pub fn circular_arc<N>(radius: N, angle: N) -> Curve<N, Vector2<N>>
where
    N: 'static + ScalarT + RealField,
{
    // normalize the angle to the range [0, 2*pi]
    let a = if angle < N::zero() {
        N::zero()
    } else if angle > N::two_pi() {
        N::two_pi()
    } else {
        angle
    };

    // find the number of arcs required
    N::floor(angle / N::frac_pi_2());


    unimplemented!()
}
