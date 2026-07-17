macro_rules! _impl_vector_tuple_unary_op {
    ($trait:ident, $method:ident $(,)?) => {
        $crate::common::_impl_tuple_wrapper_unary_op!(
            [T, const N: usize],
            $crate::vector::Vector<T, N>,
            item: T,
            len: N,
            $trait,
            $method
        );
    };
}

macro_rules! _impl_vector_tuple_binary_op {
    ($trait:ident, $method:ident $(,)?) => {
        $crate::common::_impl_tuple_wrapper_binary_op!(
            [T, const N: usize],
            $crate::vector::Vector<T, N>,
            item: T,
            len: N,
            $trait,
            $method
        );
    };

    ($trait:ident<$rhs:ty>, $method:ident $(,)?) => {
        $crate::common::_impl_tuple_wrapper_binary_op!(
            [T, const N: usize],
            $crate::vector::Vector<T, N>,
            item: T,
            len: N,
            $trait<$rhs>,
            $method
        );
    };
}

macro_rules! _impl_vector_tuple_assign_op {
    ($trait:ident, $method:ident $(,)?) => {
        $crate::common::_impl_tuple_wrapper_assign_op!(
            [T, const N: usize],
            $crate::vector::Vector<T, N>,
            item: T,
            len: N,
            $trait,
            $method
        );
    };

    ($trait:ident<$rhs:ty>, $method:ident $(,)?) => {
        $crate::common::_impl_tuple_wrapper_assign_op!(
            [T, const N: usize],
            $crate::vector::Vector<T, N>,
            item: T,
            len: N,
            $trait<$rhs>,
            $method
        );
    };
}

pub(crate) use _impl_vector_tuple_assign_op;
pub(crate) use _impl_vector_tuple_binary_op;
pub(crate) use _impl_vector_tuple_unary_op;
