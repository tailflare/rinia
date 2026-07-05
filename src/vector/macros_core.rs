macro_rules! impl_vector_core_traits {
    ($wrapper:ident<$scalar:ident : $bound:path>, $dim:expr) => {
        $crate::tuple::impl_scalar_tuple_traits!($wrapper<$scalar: $bound>, $dim);

        impl<$scalar> $crate::Vector for $wrapper<$scalar> where $scalar: $bound {}
    };
}

pub(crate) use impl_vector_core_traits;
