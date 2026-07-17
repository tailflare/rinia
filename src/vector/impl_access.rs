use crate::{Vector, common};

// Array constructor for Vector<T, N>
common::_impl_tuple_wrapper_from_array!([T, const N: usize], Vector<T, N>, item: T, len: N, field: data);

// Tuple constructor for Vector<T, N>
common::_impl_tuple_wrapper_from_tuple!([T, const N: usize], Vector<T, N>, item: T, len: N, field: data);

// Array accessors for Vector<T, N> (as_array, into_array)
common::_impl_tuple_wrapper_array_accessors!([T, const N: usize], Vector<T, N>, item: T, len: N, field: data);

// Tuple accessors for Vector<T, N> (as_tuple, as_mut_tuple)
common::_impl_tuple_wrapper_tuple_accessors!([T, const N: usize], Vector<T, N>, item: T, len: N, field: data);

// Slice accessors for Vector<T, N> (as_slice, as_mut_slice)
common::_impl_tuple_wrapper_slice_accessors!([T, const N: usize], Vector<T, N>, item: T, field: data);

// Iter accessors for Vector<T, N> (iter, iter_mut)
common::_impl_tuple_wrapper_iter_accessors!([T, const N: usize], Vector<T, N>, item: T, field: data);

// Index traits for Vector<T, N> (Index, IndexMut)
common::_impl_tuple_wrapper_index_traits!([T, const N: usize], Vector<T, N>, item: T);

// IntoIterator traits for Vector<T, N>, &Vector<T, N>, &mut Vector<T, N>
common::_impl_tuple_wrapper_into_iter_traits!([T, const N: usize], Vector<T, N>, item: T, len: N);

// From<[T; N]> trait for Vector<T, N>
common::_impl_tuple_wrapper_from_array_trait!([T, const N: usize], Vector<T, N>, item: T, len: N);

// TupleLike trait implementation for Vector<T, N>
common::_impl_tuplelike_for_wrapper!([T, const N: usize], Vector<T, N>, item: T, len: N);
