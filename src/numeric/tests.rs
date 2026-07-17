#![cfg(test)]

use alloc::vec::Vec;

use crate::numeric::{Cast, CastError, LossyCast, SaturatingCast, TryCast, TryExactCast};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Thin<T>(T);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ThickSingle<T> {
    pub value: T,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ThickMulti<T> {
    pub value1: T,
    pub value2: T,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ThickConst<T, const N: usize> {
    pub value1: T,
    pub value2: T,
}

crate::impl_numeric_casts_wrapper!(Thin<T>);

crate::impl_numeric_casts_wrapper!(ThickSingle<T>, [value]);

crate::impl_numeric_casts_wrapper!(ThickMulti<T>, [value1, value2]);

crate::impl_numeric_casts_wrapper!(
    [T, const N: usize],
    wrapper: ThickConst,
    ThickConst<T, N> => ThickConst<U, N>,
    item: T,
    fields: [value1, value2],
);

#[test]
fn transparent_casts_thin_inherent_surface() {
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
fn transparent_casts_thin_trait_surface() {
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
fn transparent_casts_thick_multi_inherent_surface() {
    let src = ThickMulti { value1: 100_i32, value2: 300_i32 };

    assert_eq!(src.cast::<i64>(), ThickMulti { value1: 100_i64, value2: 300_i64 });
    assert_eq!(ThickMulti::<i64>::cast_from(src), ThickMulti { value1: 100_i64, value2: 300_i64 });

    assert_eq!(src.lossy_cast::<u8>(), ThickMulti { value1: 100_u8, value2: 44_u8 });

    assert_eq!(
        ThickMulti::<u8>::lossy_cast_from(src),
        ThickMulti { value1: 100_u8, value2: 44_u8 }
    );

    assert_eq!(src.saturating_cast::<u8>(), ThickMulti { value1: 100_u8, value2: u8::MAX });

    assert_eq!(
        ThickMulti::<u8>::saturating_cast_from(src),
        ThickMulti { value1: 100_u8, value2: u8::MAX }
    );

    assert_eq!(
        ThickMulti { value1: 10_i32, value2: 255_i32 }.try_cast::<u8>(),
        Ok(ThickMulti { value1: 10_u8, value2: 255_u8 })
    );

    assert_eq!(
        ThickMulti::<u8>::try_cast_from(ThickMulti { value1: 10_i32, value2: 300_i32 }),
        Err(CastError::OutOfRange)
    );

    assert_eq!(
        ThickMulti::<i64>::try_exact_cast_from(ThickMulti { value1: 1_i32, value2: 2_i32 }),
        Ok(ThickMulti { value1: 1_i64, value2: 2_i64 })
    );
}

#[test]
fn transparent_casts_thick_multi_trait_surface() {
    let src = ThickMulti { value1: 7_i32, value2: 8_i32 };

    let casted = <ThickMulti<i32> as Cast<ThickMulti<i64>>>::cast(src);
    assert_eq!(casted, ThickMulti { value1: 7_i64, value2: 8_i64 });

    let lossy = <ThickMulti<i32> as LossyCast<ThickMulti<u8>>>::lossy_cast(ThickMulti {
        value1: 300_i32,
        value2: 511_i32,
    });
    assert_eq!(lossy, ThickMulti { value1: 44_u8, value2: 255_u8 });

    let saturating =
        <ThickMulti<i32> as SaturatingCast<ThickMulti<u8>>>::saturating_cast(ThickMulti {
            value1: 300_i32,
            value2: -1_i32,
        });
    assert_eq!(saturating, ThickMulti { value1: u8::MAX, value2: 0_u8 });

    let try_ok = <ThickMulti<i32> as TryCast<ThickMulti<u8>>>::try_cast(ThickMulti {
        value1: 1_i32,
        value2: 2_i32,
    });
    assert_eq!(try_ok, Ok(ThickMulti { value1: 1_u8, value2: 2_u8 }));

    let try_err = <ThickMulti<i32> as TryCast<ThickMulti<u8>>>::try_cast(ThickMulti {
        value1: 1_i32,
        value2: 300_i32,
    });
    assert_eq!(try_err, Err(CastError::OutOfRange));

    let try_exact_err =
        <ThickMulti<f32> as TryExactCast<ThickMulti<i32>>>::try_exact_cast(ThickMulti {
            value1: 1.0_f32,
            value2: 1.5_f32,
        });
    assert_eq!(try_exact_err, Err(CastError::Fractional));
}

#[test]
fn transparent_casts_thick_single_surface() {
    let src = ThickSingle { value: 300_i32 };

    assert_eq!(src.cast::<i64>(), ThickSingle { value: 300_i64 });
    assert_eq!(ThickSingle::<i64>::cast_from(src), ThickSingle { value: 300_i64 });

    assert_eq!(src.lossy_cast::<u8>(), ThickSingle { value: 44_u8 });
    assert_eq!(ThickSingle::<u8>::lossy_cast_from(src), ThickSingle { value: 44_u8 });

    assert_eq!(src.saturating_cast::<u8>(), ThickSingle { value: u8::MAX });
    assert_eq!(ThickSingle::<u8>::saturating_cast_from(src), ThickSingle { value: u8::MAX });

    assert_eq!(src.try_cast::<u8>(), Err(CastError::OutOfRange));
    assert_eq!(ThickSingle::<u8>::try_cast_from(src), Err(CastError::OutOfRange));

    assert_eq!(
        ThickSingle::<i64>::try_exact_cast_from(ThickSingle { value: 7_i32 }),
        Ok(ThickSingle { value: 7_i64 })
    );

    let trait_casted =
        <ThickSingle<i32> as Cast<ThickSingle<i64>>>::cast(ThickSingle { value: 7_i32 });
    assert_eq!(trait_casted, ThickSingle { value: 7_i64 });

    let trait_lossy = <ThickSingle<i32> as LossyCast<ThickSingle<u8>>>::lossy_cast(src);
    assert_eq!(trait_lossy, ThickSingle { value: 44_u8 });

    let trait_saturating =
        <ThickSingle<i32> as SaturatingCast<ThickSingle<u8>>>::saturating_cast(src);
    assert_eq!(trait_saturating, ThickSingle { value: u8::MAX });

    let trait_try = <ThickSingle<i32> as TryCast<ThickSingle<u8>>>::try_cast(src);
    assert_eq!(trait_try, Err(CastError::OutOfRange));

    let trait_try_exact =
        <ThickSingle<i32> as TryExactCast<ThickSingle<i64>>>::try_exact_cast(ThickSingle {
            value: 7_i32,
        });
    assert_eq!(trait_try_exact, Ok(ThickSingle { value: 7_i64 }));
}

#[test]
fn transparent_casts_thick_const_surface() {
    let src = ThickConst::<i32, 2> { value1: 7_i32, value2: 300_i32 };

    let casted: ThickConst<i64, 2> = src.cast();
    assert_eq!(casted, ThickConst { value1: 7_i64, value2: 300_i64 });
    assert_eq!(
        ThickConst::<i64, 2>::cast_from(ThickConst::<i32, 2> { value1: 7_i32, value2: 300_i32 }),
        ThickConst { value1: 7_i64, value2: 300_i64 }
    );

    let lossy: ThickConst<u8, 2> = src.lossy_cast();
    assert_eq!(lossy, ThickConst { value1: 7_u8, value2: 44_u8 });
    assert_eq!(
        ThickConst::<u8, 2>::lossy_cast_from(ThickConst::<i32, 2> {
            value1: 7_i32,
            value2: 300_i32
        }),
        ThickConst { value1: 7_u8, value2: 44_u8 }
    );

    let saturating: ThickConst<u8, 2> = src.saturating_cast();
    assert_eq!(saturating, ThickConst { value1: 7_u8, value2: u8::MAX });
    assert_eq!(
        ThickConst::<u8, 2>::saturating_cast_from(ThickConst::<i32, 2> {
            value1: 7_i32,
            value2: 300_i32,
        }),
        ThickConst { value1: 7_u8, value2: u8::MAX }
    );

    let try_err: Result<ThickConst<u8, 2>, CastError> = src.try_cast();
    assert_eq!(try_err, Err(CastError::OutOfRange));
    assert_eq!(
        ThickConst::<u8, 2>::try_cast_from(ThickConst::<i32, 2> { value1: 7_i32, value2: 300_i32 }),
        Err(CastError::OutOfRange)
    );

    let try_exact_err: Result<ThickConst<i32, 2>, CastError> =
        ThickConst::<f32, 2> { value1: 1.0_f32, value2: 1.5_f32 }.try_exact_cast();
    assert_eq!(try_exact_err, Err(CastError::Fractional));
    assert_eq!(
        ThickConst::<i32, 2>::try_exact_cast_from(ThickConst::<f32, 2> {
            value1: 1.0_f32,
            value2: 1.5_f32,
        }),
        Err(CastError::Fractional)
    );

    let trait_casted = <ThickConst<i32, 2> as Cast<ThickConst<i64, 2>>>::cast(src);
    assert_eq!(trait_casted, ThickConst { value1: 7_i64, value2: 300_i64 });

    let trait_lossy = <ThickConst<i32, 2> as LossyCast<ThickConst<u8, 2>>>::lossy_cast(src);
    assert_eq!(trait_lossy, ThickConst { value1: 7_u8, value2: 44_u8 });

    let trait_saturating =
        <ThickConst<i32, 2> as SaturatingCast<ThickConst<u8, 2>>>::saturating_cast(src);
    assert_eq!(trait_saturating, ThickConst { value1: 7_u8, value2: u8::MAX });

    let trait_try = <ThickConst<i32, 2> as TryCast<ThickConst<u8, 2>>>::try_cast(src);
    assert_eq!(trait_try, Err(CastError::OutOfRange));

    let trait_try_exact =
        <ThickConst<i32, 2> as TryExactCast<ThickConst<i64, 2>>>::try_exact_cast(ThickConst {
            value1: 1_i32,
            value2: 2_i32,
        });
    assert_eq!(trait_try_exact, Ok(ThickConst { value1: 1_i64, value2: 2_i64 }));
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
