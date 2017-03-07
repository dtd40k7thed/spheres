//! The updater pulls down the necessary data to populate the repository. The canonical repository
//! is stored in git for its flexibility of version control.

use config::Config;
use std::env;
use std::fs;
use std::fs::Path;

pub struct Updater {
	config: Config
}

impl Updater {

	pub fn new(config: Config) -> Updater {
		return Updater {
			config: config
		}
	}

	/// This pulls data from github and updates the repository with it.
	pub fn update() {
		//
	}

}

fn update() {

	let repo = Path::new(".");

	if !repo.exists() || !repo.is_dir() {
		if !repo.is_dir() {
			fs::remove_file(repo.to_string())?;
		}

		fs::create_dir(repo.to_string())?;
	}

	let hash = repo.join("hash");

	let hash_remote = unimplemented!(); // Download from github

	if !hash.exists() || hash != hash_remote {
		// Download zip from github master.

		// Unpack zip into directory.
	}
}
