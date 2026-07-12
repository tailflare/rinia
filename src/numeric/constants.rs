/// The zero constant for a type (often used as the additive identity).
pub trait Zero {
    const ZERO: Self;
}

/// The one constant for a type (often used as the multiplicative identity).
pub trait One {
    const ONE: Self;
}

/// The two constant for a type.
pub trait Two {
    const TWO: Self;
}

/// The half constant for a type.
pub trait Half {
    const HALF: Self;
}

/// The negative one constant for a type.
pub trait NegOne {
    const NEG_ONE: Self;
}

/// A trait for types that are bounded, meaning they have a minimum and maximum value.
pub trait Bounded {
    /// The minimum value of the type.
    const MIN: Self;

    /// The maximum value of the type.
    const MAX: Self;
}

/// The infinite constants for a type, representing positive and negative infinity.
pub trait Infinite {
    /// The positive infinity value.
    const INFINITY: Self;

    /// The negative infinity value.
    const NEG_INFINITY: Self;
}

/// The NaN constant for a type.
pub trait Nan {
    /// The NaN value.
    const NAN: Self;
}
