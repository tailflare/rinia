#[cfg(feature = "bytemuck")]
#[macro_export]
macro_rules! impl_bytemuck_pod {
    ($wrapper:ident<$ty:ident>) => {
        unsafe impl<$ty> ::bytemuck::Zeroable for $wrapper<$ty> where $ty: ::bytemuck::Zeroable {}

        unsafe impl<$ty> ::bytemuck::Pod for $wrapper<$ty> where $ty: ::bytemuck::Pod {}
    };

    ($wrapper:ident<$ty:ident : $bound:path>) => {
        unsafe impl<$ty> ::bytemuck::Zeroable for $wrapper<$ty> where
            $ty: $bound + ::bytemuck::Zeroable
        {
        }

        unsafe impl<$ty> ::bytemuck::Pod for $wrapper<$ty> where $ty: $bound + ::bytemuck::Pod {}
    };

    ($wrapper:ident) => {
        unsafe impl ::bytemuck::Zeroable for $wrapper {}

        unsafe impl ::bytemuck::Pod for $wrapper {}
    };
}

#[cfg(not(feature = "bytemuck"))]
#[macro_export]
macro_rules! impl_bytemuck_pod {
    ($wrapper:ident<$ty:ident>) => {};
    ($wrapper:ident<$ty:ident : $bound:path>) => {};
    ($wrapper:ident) => {};
}

#[cfg(feature = "bytemuck")]
#[macro_export]
macro_rules! impl_bytemuck_transparent {
    ($wrapper:ident<$ty:ident>, $repr:ty) => {
        $crate::impl_bytemuck_pod!($wrapper<$ty>);

        unsafe impl<$ty> ::bytemuck::TransparentWrapper<$repr>
            for $wrapper<$ty>
        {
        }
    };

    ($wrapper:ident<$ty:ident : $bound:path>, $repr:ty) => {
        $crate::impl_bytemuck_pod!($wrapper<$ty: $bound>);

        unsafe impl<$ty> ::bytemuck::TransparentWrapper<$repr>
            for $wrapper<$ty>
        where
            $ty: $bound,
        {
        }
    };

    ($wrapper:ident, $repr:ty) => {
        $crate::impl_bytemuck_pod!($wrapper);

        unsafe impl ::bytemuck::TransparentWrapper<$repr> for $wrapper {}
    };
}

#[cfg(not(feature = "bytemuck"))]
#[macro_export]
macro_rules! impl_bytemuck_transparent {
    ($wrapper:ident<$ty:ident>, $repr:ty) => {};
    ($wrapper:ident<$ty:ident : $bound:path>, $repr:ty) => {};
    ($wrapper:ident, $repr:ty) => {};
}
