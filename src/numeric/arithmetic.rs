pub trait SaturatingAdd<Rhs = Self> {
    type Output;

    fn saturating_add(self, rhs: Rhs) -> Self::Output;
}

pub trait SaturatingSub<Rhs = Self> {
    type Output;

    fn saturating_sub(self, rhs: Rhs) -> Self::Output;
}

pub trait SaturatingMul<Rhs = Self> {
    type Output;

    fn saturating_mul(self, rhs: Rhs) -> Self::Output;
}

pub trait SaturatingDiv<Rhs = Self> {
    type Output;

    fn saturating_div(self, rhs: Rhs) -> Self::Output;
}

pub trait SaturatingNeg {
    type Output;

    fn saturating_neg(self) -> Self::Output;
}

pub trait WrappingAdd<Rhs = Self> {
    type Output;

    fn wrapping_add(self, rhs: Rhs) -> Self::Output;
}

pub trait WrappingSub<Rhs = Self> {
    type Output;

    fn wrapping_sub(self, rhs: Rhs) -> Self::Output;
}

pub trait WrappingMul<Rhs = Self> {
    type Output;

    fn wrapping_mul(self, rhs: Rhs) -> Self::Output;
}

pub trait WrappingDiv<Rhs = Self> {
    type Output;

    fn wrapping_div(self, rhs: Rhs) -> Self::Output;
}

pub trait WrappingRem<Rhs = Self> {
    type Output;

    fn wrapping_rem(self, rhs: Rhs) -> Self::Output;
}

pub trait WrappingNeg {
    type Output;

    fn wrapping_neg(self) -> Self::Output;
}

pub trait CheckedAdd<Rhs = Self> {
    type Output;

    fn checked_add(self, rhs: Rhs) -> Self::Output;
}

pub trait CheckedSub<Rhs = Self> {
    type Output;

    fn checked_sub(self, rhs: Rhs) -> Self::Output;
}

pub trait CheckedMul<Rhs = Self> {
    type Output;

    fn checked_mul(self, rhs: Rhs) -> Self::Output;
}

pub trait CheckedDiv<Rhs = Self> {
    type Output;

    fn checked_div(self, rhs: Rhs) -> Self::Output;
}

pub trait CheckedRem<Rhs = Self> {
    type Output;

    fn checked_rem(self, rhs: Rhs) -> Self::Output;
}

pub trait CheckedNeg {
    type Output;

    fn checked_neg(self) -> Self::Output;
}
