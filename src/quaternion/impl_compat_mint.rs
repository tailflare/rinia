//! This module provides compatibility with the mint crate for the [Quaternion] type.
#![cfg(feature = "mint")]

use crate::quaternion::Quaternion;

impl From<mint::Quaternion<f32>> for Quaternion<f32> {
    #[inline]
    fn from(value: mint::Quaternion<f32>) -> Self {
        Self::new(value.v.x, value.v.y, value.v.z, value.s)
    }
}

impl From<Quaternion<f32>> for mint::Quaternion<f32> {
    #[inline]
    fn from(value: Quaternion<f32>) -> Self {
        let [x, y, z, w] = value.into_array();
        Self { v: mint::Vector3 { x, y, z }, s: w }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mint_to_quaternion_maps_fields() {
        let mint_q = mint::Quaternion {
            v: mint::Vector3 { x: 1.0_f32, y: 2.0_f32, z: 3.0_f32 },
            s: 4.0_f32,
        };

        let q: Quaternion<f32> = mint_q.into();
        assert_eq!(q.into_array(), [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn quaternion_to_mint_maps_fields() {
        let q = Quaternion::new(5.0_f32, 6.0_f32, 7.0_f32, 8.0_f32);
        let mint_q: mint::Quaternion<f32> = q.into();

        assert_eq!(mint_q.v.x, 5.0);
        assert_eq!(mint_q.v.y, 6.0);
        assert_eq!(mint_q.v.z, 7.0);
        assert_eq!(mint_q.s, 8.0);
    }
}
