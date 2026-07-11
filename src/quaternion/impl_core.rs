use crate::{
    common,
    quaternion::{Quaternion, QuaternionFields},
    scalar::{HasScalar, Scalar},
};

// Inherent impls for Quaternion<T>
impl<T> Quaternion<T> {
    /// Creates a new quaternion with the given components.
    #[inline]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self::from_array([x, y, z, w])
    }
}

// Array constructor for Quaternion<T>
common::impl_tuple_wrapper_from_array!([T], Quaternion<T>, item: T, len: 4, field: data);

// Tuple constructor for Quaternion<T>
common::impl_tuple_wrapper_from_tuple!([T], Quaternion<T>, item: T, len: 4, field: data);

// Array accessors for Quaternion<T> (as_array, into_array)
common::impl_tuple_wrapper_array_accessors!([T], Quaternion<T>, item: T, len: 4, field: data);

// Field accessors via Deref for Quaternion<T>
common::impl_layout_field_access!([T], Quaternion<T> => QuaternionFields<T>);

// Tuple accessors for Quaternion<T> (as_tuple, as_mut_tuple)
common::impl_tuple_wrapper_tuple_accessors!([T], Quaternion<T>, item: T, len: 4, field: data);

// Slice accessors for Quaternion<T> (as_slice, as_mut_slice)
common::impl_tuple_wrapper_slice_accessors!([T], Quaternion<T>, item: T, field: data);

// Iter accessors for Quaternion<T> (iter, iter_mut)
common::impl_tuple_wrapper_iter_accessors!([T], Quaternion<T>, item: T, field: data);

// Index traits for Quaternion<T> (Index, IndexMut)
common::impl_tuple_wrapper_index_traits!([T], Quaternion<T>, item: T);

// IntoIterator traits for Quaternion<T>, &Quaternion<T>, &mut Quaternion<T>
common::impl_tuple_wrapper_into_iter_traits!([T], Quaternion<T>, item: T, len: 4);

// From<[T; 4]> trait for Quaternion<T>
common::impl_tuple_wrapper_from_array_trait!([T], Quaternion<T>, item: T, len: 4);

// TupleLike trait implementation for Quaternion<T>
common::impl_tuplelike_for_wrapper!([T], Quaternion<T>, item: T, len: 4);

// Bytemuck trait implementations for Quaternion<T>
common::impl_bytemuck_basic!([T], Quaternion<T>, item: T);

// Blanket impl for HasScalar for Quaternion<T> where T implements Scalar
impl<T> HasScalar for Quaternion<T>
where
    T: Scalar,
{
    type Scalar = T;
}
