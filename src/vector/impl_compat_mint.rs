//! This module provides compatibility with the mint crate for the [Vector] type.
#![cfg(feature = "mint")]

use crate::vector::Vector;

macro_rules! impl_mint_vector {
    ($n:literal, $mint:ident, [$($field:ident),+]) => {
        impl<T> From<mint::$mint<T>> for Vector<T, $n> {
            #[inline]
            fn from(value: mint::$mint<T>) -> Self {
                Self::new($(value.$field),+)
            }
        }

        impl<T> From<Vector<T, $n>> for mint::$mint<T> {
            #[inline]
            fn from(value: Vector<T, $n>) -> Self {
                let [$($field),+] = value.into_array();
                Self { $($field),+ }
            }
        }
    };
}

impl_mint_vector!(2, Vector2, [x, y]);
impl_mint_vector!(3, Vector3, [x, y, z]);
impl_mint_vector!(4, Vector4, [x, y, z, w]);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector2_roundtrip() {
        let mint_v = mint::Vector2 { x: 1_i32, y: 2_i32 };
        let v: Vector<i32, 2> = mint_v.into();
        assert_eq!(v.into_array(), [1, 2]);

        let mint_back: mint::Vector2<i32> = v.into();
        assert_eq!(mint_back.x, 1);
        assert_eq!(mint_back.y, 2);
    }

    #[test]
    fn vector3_roundtrip() {
        let mint_v = mint::Vector3 { x: 3_i32, y: 4_i32, z: 5_i32 };
        let v: Vector<i32, 3> = mint_v.into();
        assert_eq!(v.into_array(), [3, 4, 5]);

        let mint_back: mint::Vector3<i32> = v.into();
        assert_eq!(mint_back.x, 3);
        assert_eq!(mint_back.y, 4);
        assert_eq!(mint_back.z, 5);
    }

    #[test]
    fn vector4_roundtrip() {
        let mint_v = mint::Vector4 { x: 6_i32, y: 7_i32, z: 8_i32, w: 9_i32 };
        let v: Vector<i32, 4> = mint_v.into();
        assert_eq!(v.into_array(), [6, 7, 8, 9]);

        let mint_back: mint::Vector4<i32> = v.into();
        assert_eq!(mint_back.x, 6);
        assert_eq!(mint_back.y, 7);
        assert_eq!(mint_back.z, 8);
        assert_eq!(mint_back.w, 9);
    }
}
