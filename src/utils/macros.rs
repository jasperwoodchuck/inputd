#[macro_export]
macro_rules! try_or {
	(continue $expr:expr) => {
		match $expr {
			Ok(value) => value,
			Err(err) => {
				::tracing::error!("{err}");
				continue;
			}
		}
	};

	(return $expr:expr) => {
		match $expr {
			Ok(value) => value,
			Err(err) => {
				::tracing::error!("{err}");
				return;
			}
		}
	};
}
