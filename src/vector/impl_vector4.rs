use crate::{
    common,
    numeric::{NegOne, One, Zero},
    vector::{Vector, Vector4Fields},
};

// Inherent impl for Vector<T, 4>
impl<T> Vector<T, 4> {
    /// Creates a new 4D vector with the given components.
    #[inline]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self::from_array([x, y, z, w])
    }
}

// Unit vectors and axes for Vector<T, 4>
impl<T> Vector<T, 4>
where
    T: Zero + One,
{
    /// The X unit vector (1, 0, 0).
    pub const X: Self = Self::new(T::ONE, T::ZERO, T::ZERO, T::ZERO);

    /// The Y unit vector (0, 1, 0).
    pub const Y: Self = Self::new(T::ZERO, T::ONE, T::ZERO, T::ZERO);

    /// The Z unit vector (0, 0, 1).
    pub const Z: Self = Self::new(T::ZERO, T::ZERO, T::ONE, T::ZERO);

    /// The W unit vector (0, 0, 0, 1).
    pub const W: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ONE);

    /// The unit axes.
    pub const AXES: [Self; 4] = [Self::X, Self::Y, Self::Z, Self::W];
}

// Negative unit vectors for Vector<T, 4>
impl<T> Vector<T, 4>
where
    T: Zero + NegOne,
{
    /// The negative X unit vector (-1, 0, 0).
    pub const NEG_X: Self = Self::new(T::NEG_ONE, T::ZERO, T::ZERO, T::ZERO);

    /// The negative Y unit vector (0, -1, 0).
    pub const NEG_Y: Self = Self::new(T::ZERO, T::NEG_ONE, T::ZERO, T::ZERO);

    /// The negative Z unit vector (0, 0, -1).
    pub const NEG_Z: Self = Self::new(T::ZERO, T::ZERO, T::NEG_ONE, T::ZERO);

    /// The negative W unit vector (0, 0, 0, -1).
    pub const NEG_W: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::NEG_ONE);
}

// Field accessors via Deref for Vector<T, 4>
common::impl_layout_field_access!([T], Vector<T, 4> => Vector4Fields<T>);
