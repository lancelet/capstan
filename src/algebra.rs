use nalgebra::base::allocator::Allocator;
use nalgebra::base::{DefaultAllocator, DimName, VectorN};
use num_traits::identities::One;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub};

/// A scalar type.
///
/// Scalars are used for things like knot locations, weights, parameter values,
/// and the scalar components of vector types.
pub trait ScalarT:
    Copy
    + PartialOrd
    + Debug
    + Add<Output = Self>
    + AddAssign
    + Mul<Output = Self>
    + MulAssign
    + Sub<Output = Self>
    + Div<Output = Self>
    + One
{
}

impl<T> ScalarT for T where
    T: Copy
        + PartialOrd
        + Debug
        + Add<Output = Self>
        + AddAssign
        + Mul<Output = Self>
        + MulAssign
        + Sub<Output = Self>
        + Div<Output = Self>
        + One
{
}

/// A vector type.
///
/// Vectors are used for 3D locations like control points and points on curves
/// or surfaces.
pub trait VectorT:
    Clone + Debug + Add<Output = Self> + Mul<<Self as VectorT>::Field, Output = Self>
{
    type Field: ScalarT;
}

impl<N, D> VectorT for VectorN<N, D>
where
    N: 'static + ScalarT,
    D: DimName,
    DefaultAllocator: Allocator<N, D>,
{
    type Field = N;
}
