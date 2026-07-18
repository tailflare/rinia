/// A trait to check if a type is normalized.
pub trait IsNormalized {
    type Tolerance: Copy;

    /// Checks if the type is normalized.
    #[allow(clippy::wrong_self_convention)]
    fn is_normalized(self) -> bool;

    /// Checks if the type is normalized within a given tolerance.
    #[allow(clippy::wrong_self_convention)]
    fn is_normalized_tol(self, tol: Self::Tolerance) -> bool;
}
