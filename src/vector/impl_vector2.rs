use core::ops::{Mul, Sub};

use crate::{
    algebra::Cross,
    common,
    numeric::{NegOne, One, Zero},
    vector::{Vector, Vector2Fields},
};

// Inherent impl for Vector<T, 2>
impl<T> Vector<T, 2> {
    /// Creates a new 2D vector with the given components.
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Self::from_array([x, y])
    }
}

// Unit vectors and axes for Vector<T, 2>
impl<T> Vector<T, 2>
where
    T: Zero + One,
{
    /// The X unit vector (1, 0).
    pub const X: Self = Self::new(T::ONE, T::ZERO);

    /// The Y unit vector (0, 1).
    pub const Y: Self = Self::new(T::ZERO, T::ONE);

    /// The unit axes.
    pub const AXES: [Self; 2] = [Self::X, Self::Y];
}

// Negative unit vectors for Vector<T, 2>
impl<T> Vector<T, 2>
where
    T: Zero + NegOne,
{
    /// The negative X unit vector (-1, 0).
    pub const NEG_X: Self = Self::new(T::NEG_ONE, T::ZERO);

    /// The negative Y unit vector (0, -1).
    pub const NEG_Y: Self = Self::new(T::ZERO, T::NEG_ONE);
}

// Field accessors via Deref for Vector<T, 2>
common::impl_layout_field_access!([T], Vector<T, 2> => Vector2Fields<T>);

// Cross inherent
impl<T> Vector<T, 2>
where
    T: Copy + Mul<Output = T> + Sub<Output = T>,
{
    /// Computes the scalar cross product of two 2D vectors.
    #[inline]
    pub fn cross(self, rhs: Self) -> T {
        self.x * rhs.y - self.y * rhs.x
    }
}

// Cross trait
impl<T> Cross for Vector<T, 2>
where
    T: Copy + Mul<Output = T> + Sub<Output = T>,
{
    type Output = T;

    #[inline]
    fn cross(self, rhs: Self) -> Self::Output {
        Vector::<T, 2>::cross(self, rhs)
    }
}
