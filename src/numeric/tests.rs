#![cfg(test)]

use alloc::vec::Vec;

use crate::numeric::{Cast, CastError, LossyCast, SaturatingCast, TryCast, TryExactCast};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Thin<T>(T);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Thick<T> {
    pub value1: T,
    pub value2: T,
}

crate::impl_numeric_casts_transparent!(Thin<T>);
crate::impl_numeric_casts_named_fields!(Thick<T>, [value1, value2]);

#[test]
fn transparent_casts_inherent_surface() {
    let src = Thin(123_i32);

    assert_eq!(src.cast::<i64>(), Thin(123_i64));
    assert_eq!(Thin::<i64>::cast_from(src), Thin(123_i64));

    assert_eq!(src.lossy_cast::<u8>(), Thin(123_u8));
    assert_eq!(Thin::<u8>::lossy_cast_from(src), Thin(123_u8));

    assert_eq!(Thin(300_i32).saturating_cast::<u8>(), Thin(u8::MAX));
    assert_eq!(Thin::<u8>::saturating_cast_from(Thin(300_i32)), Thin(u8::MAX));

    assert_eq!(Thin(255_i32).try_cast::<u8>(), Ok(Thin(255_u8)));
    assert_eq!(Thin::<u8>::try_cast_from(Thin(255_i32)), Ok(Thin(255_u8)));
    assert_eq!(Thin(256_i32).try_cast::<u8>(), Err(CastError::OutOfRange));

    assert_eq!(Thin(42_i32).try_exact_cast::<i64>(), Ok(Thin(42_i64)));
    assert_eq!(Thin::<i64>::try_exact_cast_from(Thin(42_i32)), Ok(Thin(42_i64)));
    assert_eq!(Thin(1.5_f32).try_exact_cast::<i32>(), Err(CastError::Fractional));
}

#[test]
fn transparent_casts_trait_surface() {
    let src = Thin(200_i32);

    let casted = <Thin<i32> as Cast<Thin<i64>>>::cast(src);
    assert_eq!(casted, Thin(200_i64));

    let lossy = <Thin<i32> as LossyCast<Thin<u8>>>::lossy_cast(src);
    assert_eq!(lossy, Thin(200_u8));

    let saturating = <Thin<i32> as SaturatingCast<Thin<u8>>>::saturating_cast(Thin(300_i32));
    assert_eq!(saturating, Thin(u8::MAX));

    let try_ok = <Thin<i32> as TryCast<Thin<u8>>>::try_cast(src);
    assert_eq!(try_ok, Ok(Thin(200_u8)));

    let try_err = <Thin<i32> as TryCast<Thin<u8>>>::try_cast(Thin(300_i32));
    assert_eq!(try_err, Err(CastError::OutOfRange));

    let try_exact_ok = <Thin<i32> as TryExactCast<Thin<i64>>>::try_exact_cast(Thin(7_i32));
    assert_eq!(try_exact_ok, Ok(Thin(7_i64)));

    let try_exact_err = <Thin<f32> as TryExactCast<Thin<i32>>>::try_exact_cast(Thin(2.25_f32));
    assert_eq!(try_exact_err, Err(CastError::Fractional));
}

#[test]
fn transparent_casts_named_fields_inherent_surface() {
    let src = Thick { value1: 100_i32, value2: 300_i32 };

    assert_eq!(src.cast::<i64>(), Thick { value1: 100_i64, value2: 300_i64 });
    assert_eq!(Thick::<i64>::cast_from(src), Thick { value1: 100_i64, value2: 300_i64 });

    assert_eq!(src.lossy_cast::<u8>(), Thick { value1: 100_u8, value2: 44_u8 });

    assert_eq!(Thick::<u8>::lossy_cast_from(src), Thick { value1: 100_u8, value2: 44_u8 });

    assert_eq!(src.saturating_cast::<u8>(), Thick { value1: 100_u8, value2: u8::MAX });

    assert_eq!(Thick::<u8>::saturating_cast_from(src), Thick { value1: 100_u8, value2: u8::MAX });

    assert_eq!(
        Thick { value1: 10_i32, value2: 255_i32 }.try_cast::<u8>(),
        Ok(Thick { value1: 10_u8, value2: 255_u8 })
    );

    assert_eq!(
        Thick::<u8>::try_cast_from(Thick { value1: 10_i32, value2: 300_i32 }),
        Err(CastError::OutOfRange)
    );

    assert_eq!(
        Thick::<i64>::try_exact_cast_from(Thick { value1: 1_i32, value2: 2_i32 }),
        Ok(Thick { value1: 1_i64, value2: 2_i64 })
    );
}

#[test]
fn transparent_casts_named_fields_trait_surface() {
    let src = Thick { value1: 7_i32, value2: 8_i32 };

    let casted = <Thick<i32> as Cast<Thick<i64>>>::cast(src);
    assert_eq!(casted, Thick { value1: 7_i64, value2: 8_i64 });

    let lossy = <Thick<i32> as LossyCast<Thick<u8>>>::lossy_cast(Thick {
        value1: 300_i32,
        value2: 511_i32,
    });
    assert_eq!(lossy, Thick { value1: 44_u8, value2: 255_u8 });

    let saturating = <Thick<i32> as SaturatingCast<Thick<u8>>>::saturating_cast(Thick {
        value1: 300_i32,
        value2: -1_i32,
    });
    assert_eq!(saturating, Thick { value1: u8::MAX, value2: 0_u8 });

    let try_ok =
        <Thick<i32> as TryCast<Thick<u8>>>::try_cast(Thick { value1: 1_i32, value2: 2_i32 });
    assert_eq!(try_ok, Ok(Thick { value1: 1_u8, value2: 2_u8 }));

    let try_err =
        <Thick<i32> as TryCast<Thick<u8>>>::try_cast(Thick { value1: 1_i32, value2: 300_i32 });
    assert_eq!(try_err, Err(CastError::OutOfRange));

    let try_exact_err = <Thick<f32> as TryExactCast<Thick<i32>>>::try_exact_cast(Thick {
        value1: 1.0_f32,
        value2: 1.5_f32,
    });
    assert_eq!(try_exact_err, Err(CastError::Fractional));
}

#[test]
fn vec_casts_surface() {
    let src = Vec::from([1_i32, 200, 300]);

    let casted: Vec<i64> = src.clone().cast();
    assert_eq!(casted, Vec::from([1_i64, 200, 300]));

    let lossy: Vec<u8> = src.clone().lossy_cast();
    assert_eq!(lossy, Vec::from([1_u8, 200, 44]));

    let saturating: Vec<u8> = src.clone().saturating_cast();
    assert_eq!(saturating, Vec::from([1_u8, 200, u8::MAX]));

    let try_err: Result<Vec<u8>, CastError> = src.clone().try_cast();
    assert_eq!(try_err, Err(CastError::OutOfRange));

    let try_ok: Result<Vec<u8>, CastError> = Vec::from([1_i32, 200]).try_cast();
    assert_eq!(try_ok, Ok(Vec::from([1_u8, 200])));

    let try_exact_ok: Result<Vec<i64>, CastError> = Vec::from([1_i32, 2]).try_exact_cast();
    assert_eq!(try_exact_ok, Ok(Vec::from([1_i64, 2])));

    let try_exact_err: Result<Vec<i32>, CastError> = Vec::from([1.0_f32, 1.5]).try_exact_cast();
    assert_eq!(try_exact_err, Err(CastError::Fractional));
}
