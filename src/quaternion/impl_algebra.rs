use core::ops::{Add, Div, Mul, Neg, Sub};

use crate::{
    Vector,
    algebra::{ApproxEqAbs, Dot, Identity, Inverse, Length, LengthSquared, Lerp, Normalize},
    numeric::{Clamp, NegOne, One, Sqrt, Trigonometry, Two, Zero},
    quaternion::Quaternion,
};

// Identity inherent
impl<T> Quaternion<T>
where
    T: Zero + One,
{
    /// The identity quaternion (0, 0, 0, 1).
    pub const IDENTITY: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ONE);
}

// Identity trait
impl<T> Identity for Quaternion<T>
where
    T: Zero + One,
{
    const IDENTITY: Self = Quaternion::IDENTITY;
}

crate::impl_approx_eq_wrapper!(
    [T],
    impl: Quaternion<T>,
    item: T,
    extra_bounds: [Neg<Output = T>],
    compare_abs: |lhs, rhs, tol| {
        let lhs_data = lhs.data;
        let rhs_data = rhs.data;
        lhs_data.approx_eq_abs_tol(&rhs_data, tol) || lhs_data.approx_eq_abs_tol(&-rhs_data, tol)
    },
    compare_rel: |lhs, rhs, tol| {
        let lhs_data = lhs.data;
        let rhs_data = rhs.data;
        lhs_data.approx_eq_rel_tol(&rhs_data, tol) || lhs_data.approx_eq_rel_tol(&-rhs_data, tol)
    },
);

// Length and length squared inherent
impl<T> Quaternion<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T>,
{
    /// Computes the squared length of the quaternion.
    #[inline]
    pub fn length_squared(self) -> T {
        self.into_tuple().length_squared()
    }

    /// Computes the length (magnitude) of the quaternion.
    #[inline]
    pub fn length(self) -> T
    where
        T: Sqrt,
    {
        self.length_squared().sqrt()
    }

    /// Returns a normalized quaternion with length one.
    #[inline]
    pub fn normalize(self) -> Self
    where
        T: Sqrt + Div<Output = T>,
    {
        self / self.length()
    }
}

// Length squared trait
impl<T> LengthSquared for Quaternion<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T>,
{
    type Output = T;

    #[inline]
    fn length_squared(self) -> Self::Output {
        Quaternion::length_squared(self)
    }
}

// Length trait
impl<T> Length for Quaternion<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + Sqrt,
{
    #[inline]
    fn length(self) -> Self::Output {
        Quaternion::length(self)
    }
}

// Normalize trait
impl<T> Normalize for Quaternion<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + Div<Output = T> + Sqrt,
{
    #[inline]
    fn normalize(self) -> Self {
        Quaternion::normalize(self)
    }
}

// Dot inherent
impl<T> Quaternion<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T>,
{
    /// Computes the dot product of two quaternions.
    #[inline]
    pub fn dot(self, rhs: Self) -> T {
        self.into_tuple().dot(rhs.into_tuple())
    }
}

// Dot trait
impl<T> Dot for Quaternion<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T>,
{
    type Output = T;

    #[inline]
    fn dot(self, rhs: Self) -> Self::Output {
        Quaternion::dot(self, rhs)
    }
}

// Conjugate inherent
impl<T> Quaternion<T>
where
    T: Copy + Neg<Output = T>,
{
    /// Returns the conjugate of the quaternion.
    #[inline]
    pub fn conjugate(self) -> Self {
        Self::new(-self.x, -self.y, -self.z, self.w)
    }
}

// Inverse inherent
impl<T> Quaternion<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + Div<Output = T> + Neg<Output = T>,
{
    /// Returns the inverse of the quaternion.
    #[inline]
    pub fn inverse(self) -> Self {
        self.conjugate() / self.length_squared()
    }
}

// Inverse trait
impl<T> Inverse for Quaternion<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + Div<Output = T> + Neg<Output = T>,
{
    #[inline]
    fn inverse(self) -> Self {
        Quaternion::inverse(self)
    }
}

// Compose inherent
impl<T> Quaternion<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + Sub<Output = T>,
{
    /// Composes two quaternions, returning a quaternion representing
    /// the combined rotation.
    #[inline]
    pub fn compose(self, rhs: Self) -> Self {
        let (x1, y1, z1, w1) = (self.x, self.y, self.z, self.w);
        let (x2, y2, z2, w2) = (rhs.x, rhs.y, rhs.z, rhs.w);

        let x = w1 * x2 + x1 * w2 + y1 * z2 - z1 * y2;
        let y = w1 * y2 - x1 * z2 + y1 * w2 + z1 * x2;
        let z = w1 * z2 + x1 * y2 - y1 * x2 + z1 * w2;
        let w = w1 * w2 - x1 * x2 - y1 * y2 - z1 * z2;

        Self::new(x, y, z, w)
    }
}

// Rotate vector inherent
impl<T> Quaternion<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Two,
{
    /// Rotates a vector by this quaternion.
    #[inline]
    pub fn rotate_vector(self, rhs: Vector<T, 3>) -> Vector<T, 3> {
        let u = Vector::<T, 3>::new(self.x, self.y, self.z);
        let w = self.w;

        let uv = u.cross(rhs);
        let uuv = u.cross(uv);

        rhs + uv * (w * T::TWO) + uuv * T::TWO
    }
}

// Lerp inherent
impl<T> Quaternion<T>
where
    T: Copy + Sub<Output = T> + Mul<Output = T> + Add<Output = T>,
{
    /// Performs linear interpolation between two quaternions.
    ///
    /// The result is not guaranteed to be normalized.
    #[inline]
    pub fn lerp(self, rhs: Self, t: T) -> Self {
        self + (rhs - self) * t
    }
}

// Lerp trait
impl<T> Lerp for Quaternion<T>
where
    T: Copy + Sub<Output = T> + Mul<Output = T> + Add<Output = T>,
{
    type Scalar = T;

    #[inline]
    fn lerp(self, rhs: Self, t: Self::Scalar) -> Self {
        Quaternion::lerp(self, rhs, t)
    }
}

// Nlerp inherent
impl<T> Quaternion<T>
where
    T: Copy
        + Sub<Output = T>
        + Mul<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + Neg<Output = T>
        + Zero
        + Sqrt
        + PartialOrd,
{
    /// Performs normalized linear interpolation between two quaternions.
    #[inline]
    pub fn nlerp(self, mut rhs: Self, t: T) -> Self {
        if self.dot(rhs) < T::ZERO {
            rhs = -rhs;
        }

        self.lerp(rhs, t).normalize()
    }
}

impl<T> Quaternion<T>
where
    T: Copy
        + Sub<Output = T>
        + Add<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Neg<Output = T>
        + Sqrt
        + Trigonometry
        + Zero
        + One
        + NegOne
        + Clamp
        + PartialOrd
        + ApproxEqAbs<Tolerance = T>,
{
    /// Performs spherical linear interpolation between two quaternions.
    ///
    /// The shortest rotation path is used.
    #[inline]
    pub fn slerp(self, mut rhs: Self, t: T) -> Self {
        let mut dot = self.dot(rhs);

        // Take shortest path.
        if dot < T::ZERO {
            rhs = -rhs;
            dot = -dot;
        }

        // Protect acos from floating point drift.
        dot = dot.clamp_value(T::NEG_ONE, T::ONE);

        // Avoid instability when the angle is very small.
        if dot.approx_eq_abs(&T::ONE) {
            return self.nlerp(rhs, t);
        }

        let theta = dot.acos();
        let sin_theta = theta.sin();

        let a = ((T::ONE - t) * theta).sin() / sin_theta;
        let b = (t * theta).sin() / sin_theta;

        self * a + rhs * b
    }
}

// From axis-angle inherent
impl<T> Quaternion<T>
where
    T: Copy + Mul<Output = T> + Div<Output = T> + Trigonometry + Two,
{
    /// Creates a quaternion from an axis and rotation angle.
    ///
    /// The axis is expected to be normalized.
    #[inline]
    pub fn from_axis_angle(axis: Vector<T, 3>, angle: T) -> Self {
        let half_angle = angle / T::TWO;
        let (sin, cos) = half_angle.sin_cos();

        let axis = axis * sin;

        Self::new(axis.x, axis.y, axis.z, cos)
    }
}

// To axis-angle inherent
impl<T> Quaternion<T>
where
    T: Copy
        + Mul<Output = T>
        + Div<Output = T>
        + Trigonometry
        + Zero
        + One
        + Two
        + NegOne
        + Clamp
        + ApproxEqAbs<Tolerance = T>,
{
    /// Returns the rotation axis and angle.
    ///
    /// The quaternion is expected to be normalized.
    #[inline]
    pub fn to_axis_angle(self) -> (Vector<T, 3>, T) {
        let half_angle = self.w.clamp_value(T::NEG_ONE, T::ONE).acos();
        let angle = half_angle * T::TWO;

        let sin_half_angle = half_angle.sin();

        // Identity rotation has no unique axis.
        if sin_half_angle.approx_eq_abs(&T::ZERO) {
            return (Vector::<T, 3>::new(T::ONE, T::ZERO, T::ZERO), T::ZERO);
        }

        let axis = Vector::<T, 3>::new(self.x, self.y, self.z) / sin_half_angle;

        (axis, angle)
    }
}
