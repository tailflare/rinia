use crate::scalar::{HasScalar, Scalar};

// Blanket impl for HasScalar for all scalar types.
impl<T> HasScalar for T
where
    T: Scalar,
{
    type Scalar = T;
}
