macro_rules! _impl_marker_trait {
	($trait:path, [$($ty:ty),+ $(,)?]) => {
		$(
			impl $trait for $ty {}
		)+
	};
}

pub(crate) use _impl_marker_trait;
