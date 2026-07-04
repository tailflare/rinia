macro_rules! define_float_repr_trait {
    ($($float:ident),+ $(,)?) => {
        pastey::paste! {
            /// Conversion surface between supported float representations.
            pub trait FloatRepr: Copy {
                $(
                    #[doc = concat!("Converts this value to `", stringify!($float), "`.")]
                    #[doc = ""]
                    #[doc = "Lossy when precision differs."]
                    fn [<as_ $float>](self) -> $float;
                )+

                $(
                    #[doc = concat!("Constructs `Self` from `", stringify!($float), "`.")]
                    #[doc = ""]
                    #[doc = "Lossy when precision differs."]
                    fn [<from_ $float>](value: $float) -> Self;
                )+

                /// Converts from any `FloatRepr` into `Self`.
                ///
                /// Lossy when precision differs.
                fn from_float<V: FloatRepr>(value: V) -> Self;
            }
        }
    };
}

pub(crate) use define_float_repr_trait;

macro_rules! impl_float_repr {
    ($target:ident, [$($float:ident),+ $(,)?]) => {
        pastey::paste! {
            impl $crate::FloatRepr for $target {
                $(
                    #[inline]
                    fn [<as_ $float>](self) -> $float {
                        self as $float
                    }
                )+

                $(
                    #[inline]
                    fn [<from_ $float>](value: $float) -> Self {
                        value as Self
                    }
                )+

                #[inline]
                fn from_float<V: $crate::FloatRepr>(value: V) -> Self {
                    value.[<as_ $target>]()
                }
            }
        }
    };
}

pub(crate) use impl_float_repr;
