//! This module manages the local configuration settings.

pub struct Config {
}

impl Config {

	pub fn new() -> Config {
		return Config {
		};
	}

	/// Provides access to the configuration settings of the application.
	pub fn fetch_str(&self, path: &'static str) -> Option<&str> {
		return None;
	}

}
