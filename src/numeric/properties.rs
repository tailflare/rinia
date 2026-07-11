/// A trait for types that are bounded, meaning they have a minimum and maximum value.
pub trait Bounded {
    /// The minimum value of the type.
    const MIN: Self;

    /// The maximum value of the type.
    const MAX: Self;
}
