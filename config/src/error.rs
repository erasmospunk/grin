// Copyright 2019 The Grin Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Error types for config
use failure::{Context, Fail};
use std::fmt::{self, Display};
use std::io;

/// Error definition
#[derive(Debug, Fail)]
pub struct ConfigError {
	inner: Context<ErrorKind>,
}

/// Config error definitions
#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
	/// Error with parsing of config file
	#[fail(display = "Error parsing configuration file at {} - {}", _0, _1)]
	ParseError(String, String),

	/// Error with fileIO while reading config file
	#[fail(display = "Error loading config file at {}", path)]
	FileIOError { path: String },

	/// No file found
	#[fail(display = "Configuration file not found: {}", _0)]
	FileNotFoundError(String),

	/// Error serializing config values
	#[fail(display = "Error serializing configuration: {}", _0)]
	SerializationError(String),
}

impl Display for ConfigError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		Display::fmt(&self.inner, f)
	}
}

impl ConfigError {
	/// Return errorkind
	pub fn kind(&self) -> ErrorKind {
		self.inner.get_context().clone()
	}
}

impl From<ErrorKind> for ConfigError {
	fn from(kind: ErrorKind) -> ConfigError {
		ConfigError {
			inner: Context::new(kind),
		}
	}
}

impl From<Context<ErrorKind>> for ConfigError {
	fn from(inner: Context<ErrorKind>) -> ConfigError {
		ConfigError { inner }
	}
}

impl From<toml::de::Error> for ConfigError {
	fn from(error: toml::de::Error) -> ConfigError {
		ConfigError {
			inner: Context::new(ErrorKind::ParseError("".to_string(), error.to_string())),
		}
	}
}

impl From<io::Error> for ConfigError {
	fn from(error: io::Error) -> ConfigError {
		ConfigError {
			inner: Context::new(ErrorKind::FileIOError {
				path: String::from(format!("!!!Error loading config file: {}", error)),
			}),
		}
	}
}
//
//impl From<io::Error> for ConfigError {
//	fn from(error: io::Error) -> ConfigError {
//		ConfigError {
//			inner: Context::new(ErrorKind::FileIOError(
//				String::from(""),
//				String::from(format!("Error loading config file: {}", error)),
//			)),
//		}
//	}
//}
