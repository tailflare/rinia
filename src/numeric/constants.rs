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
