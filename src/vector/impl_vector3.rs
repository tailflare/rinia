use core::ops::{Mul, Sub};

use crate::{
    algebra::Cross,
    common,
    numeric::{NegOne, One, Zero},
    vector::{Vector, Vector3Fields},
};

// Inherent impl for Vector<T, 3>
impl<T> Vector<T, 3> {
    /// Creates a new 3D vector with the given components.
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self::from_array([x, y, z])
    }
}

// Unit vectors and axes for Vector<T, 3>
impl<T> Vector<T, 3>
where
    T: Zero + One,
{
    /// The X unit vector (1, 0, 0).
    pub const X: Self = Self::new(T::ONE, T::ZERO, T::ZERO);

    /// The Y unit vector (0, 1, 0).
    pub const Y: Self = Self::new(T::ZERO, T::ONE, T::ZERO);

    /// The Z unit vector (0, 0, 1).
    pub const Z: Self = Self::new(T::ZERO, T::ZERO, T::ONE);

    /// The unit axes.
    pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];
}

// Negative unit vectors for Vector<T, 3>
impl<T> Vector<T, 3>
where
    T: Zero + NegOne,
{
    /// The negative X unit vector (-1, 0, 0).
    pub const NEG_X: Self = Self::new(T::NEG_ONE, T::ZERO, T::ZERO);

    /// The negative Y unit vector (0, -1, 0).
    pub const NEG_Y: Self = Self::new(T::ZERO, T::NEG_ONE, T::ZERO);

    /// The negative Z unit vector (0, 0, -1).
    pub const NEG_Z: Self = Self::new(T::ZERO, T::ZERO, T::NEG_ONE);
}

// Field accessors via Deref for Vector<T, 3>
common::impl_layout_field_access!([T], Vector<T, 3> => Vector3Fields<T>);

// Cross
impl<T> Vector<T, 3>
where
    T: Copy + Mul<Output = T> + Sub<Output = T>,
{
    /// Computes the cross product of two 3D vectors.
    #[inline]
    pub fn cross(self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl<T> Cross for Vector<T, 3>
where
    T: Copy + Mul<Output = T> + Sub<Output = T>,
{
    type Output = Self;

    #[inline]
    fn cross(self, rhs: Self) -> Self::Output {
        Vector::<T, 3>::cross(self, rhs)
    }
}
