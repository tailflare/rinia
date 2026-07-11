/// A trait for linear interpolation between two values.
pub trait Lerp {
    /// The scalar type used for interpolation.
    type Scalar;

    /// Linearly interpolates between `self` and `rhs` by a factor of `t`.
    fn lerp(self, rhs: Self, t: Self::Scalar) -> Self;
}
