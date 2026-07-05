use crate::{Scalar, VectorScalar, ops::Negate};

impl<T> VectorScalar for T where T: Scalar + Negate {}
