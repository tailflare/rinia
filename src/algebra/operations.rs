/// A trait that represents a type with a squared magnitude.
pub trait LengthSquared {
    type Output;

    fn length_squared(self) -> Self::Output;
}

/// A trait that represents a type with a magnitude.
pub trait Length: LengthSquared {
    fn length(self) -> Self::Output;
}

/// A trait that represents a type that can be normalized.
pub trait Normalize {
    fn normalize(self) -> Self;
}

/// A trait that represents a type which can be dotted with another type.
pub trait Dot<Rhs = Self> {
    type Output;

    fn dot(self, rhs: Rhs) -> Self::Output;
}

/// A trait that represents a type which can be crossed with another type.
pub trait Cross<Rhs = Self> {
    type Output;

    fn cross(self, rhs: Rhs) -> Self::Output;
}

/// A trait that represents a type which has an inverse.
pub trait Inverse: Sized {
    /// Returns the inverse of the type.
    fn inverse(self) -> Self;
}
