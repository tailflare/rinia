macro_rules! impl_marker_trait {
	($trait:path, [$($ty:ty),+ $(,)?]) => {
		$(
			impl $trait for $ty {}
		)+
	};
}

pub(crate) use impl_marker_trait;
