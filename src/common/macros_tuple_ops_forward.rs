macro_rules! impl_tuple_wrapper_unary_op {
    ([$($impl_generics:tt)*], $outer:ty, item: $item:ty, len: $len:expr, $trait:ident, $method:ident $(,)?) => {
        impl<$($impl_generics)*> core::ops::$trait for $outer
        where
            $crate::tuple::Tuple<$item, $len>: core::ops::$trait<Output = $crate::tuple::Tuple<$item, $len>>,
        {
            type Output = Self;

            #[inline]
            fn $method(self) -> Self::Output {
                let out = core::ops::$trait::$method(self.into_tuple());
                Self::from_tuple(out)
            }
        }
    };
}

macro_rules! impl_tuple_wrapper_binary_op {
    ([$($impl_generics:tt)*], $outer:ty, item: $item:ty, len: $len:expr, $trait:ident, $method:ident $(,)?) => {
        impl<$($impl_generics)*> core::ops::$trait for $outer
        where
            $crate::tuple::Tuple<$item, $len>: core::ops::$trait<Output = $crate::tuple::Tuple<$item, $len>>,
        {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: Self) -> Self::Output {
                let lhs = self.into_tuple();
                let rhs = rhs.into_tuple();
                let out = core::ops::$trait::$method(lhs, rhs);
                Self::from_tuple(out)
            }
        }
    };

    ([$($impl_generics:tt)*], $outer:ty, item: $item:ty, len: $len:expr, $trait:ident<$rhs:ty>, $method:ident $(,)?) => {
        impl<$($impl_generics)*> core::ops::$trait<$rhs> for $outer
        where
            $crate::tuple::Tuple<$item, $len>: core::ops::$trait<$rhs, Output = $crate::tuple::Tuple<$item, $len>>,
        {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: $rhs) -> Self::Output {
                let lhs = self.into_tuple();
                let out = core::ops::$trait::$method(lhs, rhs);
                Self::from_tuple(out)
            }
        }
    };
}

macro_rules! impl_tuple_wrapper_assign_op {
    ([$($impl_generics:tt)*], $outer:ty, item: $item:ty, len: $len:expr, $trait:ident, $method:ident $(,)?) => {
        impl<$($impl_generics)*> core::ops::$trait for $outer
        where
            $crate::tuple::Tuple<$item, $len>: core::ops::$trait,
        {
            #[inline]
            fn $method(&mut self, rhs: Self) {
                let rhs = rhs.into_tuple();
                core::ops::$trait::$method(self.as_mut_tuple(), rhs);
            }
        }
    };

    ([$($impl_generics:tt)*], $outer:ty, item: $item:ty, len: $len:expr, $trait:ident<$rhs:ty>, $method:ident $(,)?) => {
        impl<$($impl_generics)*> core::ops::$trait<$rhs> for $outer
        where
            $crate::tuple::Tuple<$item, $len>: core::ops::$trait<$rhs>,
        {
            #[inline]
            fn $method(&mut self, rhs: $rhs) {
                core::ops::$trait::<$rhs>::$method(self.as_mut_tuple(), rhs);
            }
        }
    };
}

pub(crate) use impl_tuple_wrapper_assign_op;
pub(crate) use impl_tuple_wrapper_binary_op;
pub(crate) use impl_tuple_wrapper_unary_op;
