use crate::vector;

// -Vector<T, N> = Vector<T, N>
vector::_impl_vector_tuple_unary_op!(Neg, neg);

// Vector<T, N> + Vector<T, N> = Vector<T, N>
vector::_impl_vector_tuple_binary_op!(Add, add);

// Vector<T, N> += Vector<T, N>
vector::_impl_vector_tuple_assign_op!(AddAssign, add_assign);

// Vector<T, N> - Vector<T, N> = Vector<T, N>
vector::_impl_vector_tuple_binary_op!(Sub, sub);

// Vector<T, N> -= Vector<T, N>
vector::_impl_vector_tuple_assign_op!(SubAssign, sub_assign);

// Vector<T, N> * T = Vector<T, N>
vector::_impl_vector_tuple_binary_op!(Mul<T>, mul);

// Vector<T, N> *= T
vector::_impl_vector_tuple_assign_op!(MulAssign<T>, mul_assign);

// Vector<T, N> / T = Vector<T, N>
vector::_impl_vector_tuple_binary_op!(Div<T>, div);

// Vector<T, N> /= T
vector::_impl_vector_tuple_assign_op!(DivAssign<T>, div_assign);
