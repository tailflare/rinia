macro_rules! impl_tuple_unary_op {
	($trait:ident, $method:ident, $op:tt) => {
		impl<T, const N: usize> core::ops::$trait for $crate::tuple::Tuple<T, N>
		where
			T: core::ops::$trait<Output = T> + Copy,
		{
			type Output = Self;

			#[inline]
			fn $method(self) -> Self::Output {
				self.map(|a| $op a)
			}
		}
	};
}

macro_rules! impl_tuple_binary_op {
	($trait:ident, $method:ident, $op:tt) => {
		impl<T, const N: usize> $trait for $crate::tuple::Tuple<T, N>
		where
			T: $trait<Output = T> + Copy,
		{
			type Output = Self;

			#[inline]
			fn $method(self, rhs: Self) -> Self::Output {
				self.zip_with(&rhs, |a, b| *a $op *b)
			}
		}
	};

	($trait:ident<$rhs:ty>, $method:ident, $op:tt) => {
		impl<T, const N: usize> $trait<$rhs> for $crate::tuple::Tuple<T, N>
		where
			T: $trait<$rhs, Output = T> + Copy,
			$rhs: Copy,
		{
			type Output = Self;

			#[inline]
			fn $method(self, rhs: $rhs) -> Self::Output {
				self.map(|a| a $op rhs)
			}
		}
	};
}

macro_rules! impl_tuple_assign_op {
	($trait:ident, $method:ident, $op:tt) => {
		impl<T, const N: usize> $trait for $crate::tuple::Tuple<T, N>
		where
			T: $trait + Copy,
		{
			#[inline]
			fn $method(&mut self, rhs: Self) {
				for (lhs, rhs) in self.iter_mut().zip(rhs.inner) {
					*lhs $op rhs;
				}
			}
		}
	};

	($trait:ident<$rhs:ty>, $method:ident, $op:tt) => {
		impl<T, const N: usize> $trait<$rhs> for $crate::tuple::Tuple<T, N>
		where
			T: $trait<$rhs> + Copy,
			$rhs: Copy,
		{
			#[inline]
			fn $method(&mut self, rhs: $rhs) {
				for lhs in self.iter_mut() {
					*lhs $op rhs;
				}
			}
		}
	};
}

pub(crate) use impl_tuple_assign_op;
pub(crate) use impl_tuple_binary_op;
pub(crate) use impl_tuple_unary_op;
