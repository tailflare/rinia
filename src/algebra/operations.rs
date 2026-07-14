/// A trait that represents a type with a squared magnitude.
pub trait LengthSquared {
    type Output;

    /// Returns the squared length (magnitude) of the type.
    fn length_squared(self) -> Self::Output;
}

/// A trait that represents a type with a magnitude.
pub trait Length: LengthSquared {
    /// Returns the length (magnitude) of the type.
    fn length(self) -> Self::Output;
}

/// A trait that represents a type that can be normalized.
pub trait Normalize {
    /// Returns a normalized version of the type.
    fn normalize(self) -> Self;
}

/// A trait that represents a type which can be dotted with another type.
pub trait Dot<Rhs = Self> {
    type Output;

    /// Computes the dot product of two vectors.
    fn dot(self, rhs: Rhs) -> Self::Output;
}

/// A trait that represents a type which can be crossed with another type.
pub trait Cross<Rhs = Self> {
    type Output;

    /// Computes the cross product of two vectors.
    fn cross(self, rhs: Rhs) -> Self::Output;
}

/// A trait that represents a type which has an inverse.
pub trait Inverse: Sized {
    /// Returns the inverse of the type.
    fn inverse(self) -> Self;
}
