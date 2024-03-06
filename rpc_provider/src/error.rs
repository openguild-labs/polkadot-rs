use core::fmt::Debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	SerdeJson(serde_json::error::Error),
	ExtrinsicFailed(String),
	MpscSend(String),
	InvalidUrl(String),
	RecvError(String),
	Io(String),
	MaxConnectionAttemptsExceeded,
	ConnectionClosed,
	Client(Box<dyn Debug + Send + Sync + 'static>),
}

impl From<serde_json::error::Error> for Error {
	fn from(error: serde_json::error::Error) -> Self {
		Self::SerdeJson(error)
	}
}

#[cfg(feature = "std")]
#[allow(unused_imports)]
pub use std_only::*;
#[cfg(feature = "std")]
mod std_only {
	use super::*;
	use std::sync::mpsc::{RecvError, SendError};

	impl From<SendError<String>> for Error {
		fn from(error: SendError<String>) -> Self {
			Self::MpscSend(error.0)
		}
	}

	impl From<RecvError> for Error {
		fn from(error: RecvError) -> Self {
			Self::RecvError(format!("{error:?}"))
		}
	}

	impl From<std::io::Error> for Error {
		fn from(error: std::io::Error) -> Self {
			Self::Io(format!("{error:?}"))
		}
	}

	impl From<url::ParseError> for Error {
		fn from(error: url::ParseError) -> Self {
			Self::InvalidUrl(format!("{error:?}"))
		}
	}
}
