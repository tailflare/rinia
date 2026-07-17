use core::ops::{Add, Mul, MulAssign, Sub};

use crate::{Vector, common, numeric::Two, quaternion::Quaternion};

// -Quaternion<T> = Quaternion<T>
common::_impl_tuple_wrapper_unary_op!([T], Quaternion<T>, item: T, len: 4, Neg, neg);

// Quaternion<T> + Quaternion<T> = Quaternion<T>
common::_impl_tuple_wrapper_binary_op!([T], Quaternion<T>, item: T, len: 4, Add, add);

// Quaternion<T> += Quaternion<T>
common::_impl_tuple_wrapper_assign_op!([T], Quaternion<T>, item: T, len: 4, AddAssign, add_assign);

// Quaternion<T> - Quaternion<T> = Quaternion<T>
common::_impl_tuple_wrapper_binary_op!([T], Quaternion<T>, item: T, len: 4, Sub, sub);

// Quaternion<T> -= Quaternion<T>
common::_impl_tuple_wrapper_assign_op!([T], Quaternion<T>, item: T, len: 4, SubAssign, sub_assign);

// Quaternion<T> * T = Quaternion<T>
common::_impl_tuple_wrapper_binary_op!([T], Quaternion<T>, item: T, len: 4, Mul<T>, mul);

// Quaternion<T> *= T
common::_impl_tuple_wrapper_assign_op!([T], Quaternion<T>, item: T, len: 4, MulAssign<T>, mul_assign);

// Quaternion<T> / T = Quaternion<T>
common::_impl_tuple_wrapper_binary_op!([T], Quaternion<T>, item: T, len: 4, Div<T>, div);

// Quaternion<T> /= T
common::_impl_tuple_wrapper_assign_op!([T], Quaternion<T>, item: T, len: 4, DivAssign<T>, div_assign);

// Quaternion<T> * Quaternion<T> = Quaternion<T> (compose)
impl<T> Mul for Quaternion<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + Sub<Output = T>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.compose(rhs)
    }
}

// Quaternion<T> *= Quaternion<T> (compose)
impl<T> MulAssign for Quaternion<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + Sub<Output = T>,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.compose(rhs);
    }
}

// Quaternion<T> * Vector<T, 3> = Vector<T, 3> (rotate_vector)
impl<T> Mul<Vector<T, 3>> for Quaternion<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Two,
{
    type Output = Vector<T, 3>;

    #[inline]
    fn mul(self, rhs: Vector<T, 3>) -> Self::Output {
        self.rotate_vector(rhs)
    }
}
