/// The zero constant for a type (often used as the additive identity).
pub trait Zero {
    /// The zero constant for the type.
    const ZERO: Self;
}

/// The one constant for a type (often used as the multiplicative identity).
pub trait One {
    /// The one constant for the type.
    const ONE: Self;
}

/// The two constant for a type.
pub trait Two {
    /// The two constant for the type.
    const TWO: Self;
}

/// The half constant for a type.
pub trait Half {
    /// The half constant for the type.
    const HALF: Self;
}

/// The negative one constant for a type.
pub trait NegOne {
    /// The negative one constant for the type.
    const NEG_ONE: Self;
}

/// A trait for types which have a bounded minimum value.
pub trait BoundedMin {
    /// The minimum value of the type.
    const MIN: Self;
}

/// A trait for types which have a bounded maximum value.
pub trait BoundedMax {
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
