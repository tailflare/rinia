use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

use crate::tuple::{impl_tuple_assign_op, impl_tuple_binary_op, impl_tuple_unary_op};

// -Tuple<T> = Tuple<T>
impl_tuple_unary_op!(Neg, neg, -);

// Tuple<T> + Tuple<T> = Tuple<T>
impl_tuple_binary_op!(Add, add, +);
// Tuple<T> + T = Tuple<T>
impl_tuple_binary_op!(Add<T>, add, +);
// Tuple<T> += Tuple<T>
impl_tuple_assign_op!(AddAssign, add_assign, +=);
// Tuple<T> += T
impl_tuple_assign_op!(AddAssign<T>, add_assign, +=);

// Tuple<T> - Tuple<T> = Tuple<T>
impl_tuple_binary_op!(Sub, sub, -);
// Tuple<T> - T = Tuple<T>
impl_tuple_binary_op!(Sub<T>, sub, -);
// Tuple<T> -= Tuple<T>
impl_tuple_assign_op!(SubAssign, sub_assign, -=);
// Tuple<T> -= T
impl_tuple_assign_op!(SubAssign<T>, sub_assign, -=);

// Tuple<T> * Tuple<T> = Tuple<T>
impl_tuple_binary_op!(Mul, mul, *);
// Tuple<T> * T = Tuple<T>
impl_tuple_binary_op!(Mul<T>, mul, *);
// Tuple<T> *= Tuple<T>
impl_tuple_assign_op!(MulAssign, mul_assign, *=);
// Tuple<T> *= T
impl_tuple_assign_op!(MulAssign<T>, mul_assign, *=);

// Tuple<T> / Tuple<T> = Tuple<T>
impl_tuple_binary_op!(Div, div, /);
// Tuple<T> / T = Tuple<T>
impl_tuple_binary_op!(Div<T>, div, /);
// Tuple<T> /= Tuple<T>
impl_tuple_assign_op!(DivAssign, div_assign, /=);
// Tuple<T> /= T
impl_tuple_assign_op!(DivAssign<T>, div_assign, /=);

// Tuple<T> % Tuple<T> = Tuple<T>
impl_tuple_binary_op!(Rem, rem, %);
// Tuple<T> % T = Tuple<T>
impl_tuple_binary_op!(Rem<T>, rem, %);
// Tuple<T> %= Tuple<T>
impl_tuple_assign_op!(RemAssign, rem_assign, %=);
// Tuple<T> %= T
impl_tuple_assign_op!(RemAssign<T>, rem_assign, %=);
