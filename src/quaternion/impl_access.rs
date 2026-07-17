use crate::{Quaternion, common, quaternion::QuaternionFields};

// Array constructor for Quaternion<T>
common::_impl_tuple_wrapper_from_array!([T], Quaternion<T>, item: T, len: 4, field: data);

// Tuple constructor for Quaternion<T>
common::_impl_tuple_wrapper_from_tuple!([T], Quaternion<T>, item: T, len: 4, field: data);

// Array accessors for Quaternion<T> (as_array, into_array)
common::_impl_tuple_wrapper_array_accessors!([T], Quaternion<T>, item: T, len: 4, field: data);

// Field accessors via Deref for Quaternion<T>
common::_impl_layout_field_access!([T], Quaternion<T> => QuaternionFields<T>);

// Tuple accessors for Quaternion<T> (as_tuple, as_mut_tuple)
common::_impl_tuple_wrapper_tuple_accessors!([T], Quaternion<T>, item: T, len: 4, field: data);

// Slice accessors for Quaternion<T> (as_slice, as_mut_slice)
common::_impl_tuple_wrapper_slice_accessors!([T], Quaternion<T>, item: T, field: data);

// Iter accessors for Quaternion<T> (iter, iter_mut)
common::_impl_tuple_wrapper_iter_accessors!([T], Quaternion<T>, item: T, field: data);

// Index traits for Quaternion<T> (Index, IndexMut)
common::_impl_tuple_wrapper_index_traits!([T], Quaternion<T>, item: T);

// IntoIterator traits for Quaternion<T>, &Quaternion<T>, &mut Quaternion<T>
common::_impl_tuple_wrapper_into_iter_traits!([T], Quaternion<T>, item: T, len: 4);

// From<[T; 4]> trait for Quaternion<T>
common::_impl_tuple_wrapper_from_array_trait!([T], Quaternion<T>, item: T, len: 4);

// TupleLike trait implementation for Quaternion<T>
common::_impl_tuplelike_for_wrapper!([T], Quaternion<T>, item: T, len: 4);
