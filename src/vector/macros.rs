macro_rules! impl_vector_traits {
    ($wrapper:ident<$scalar:ident : $bound:path>, $dim:expr) => {
        $crate::vector::impl_vector_core_traits!($wrapper<$scalar: $bound>, $dim);
        $crate::vector::impl_vector_arith_traits!($wrapper<$scalar: $bound>);
        $crate::vector::impl_vector_ops_traits!($wrapper<$scalar: $bound>);
        $crate::vector::impl_vector_equality_traits!($wrapper<$scalar: $bound>);
    };
}

pub(crate) use impl_vector_traits;
