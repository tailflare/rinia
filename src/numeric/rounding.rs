pub trait Rounding: Floor + Ceil + Trunc + Round + Fract {}

pub trait Floor {
    fn floor(self) -> Self;
}

pub trait Ceil {
    fn ceil(self) -> Self;
}

pub trait Trunc {
    fn trunc(self) -> Self;
}

pub trait Round {
    fn round(self) -> Self;
}

pub trait Fract {
    fn fract(self) -> Self;
}
