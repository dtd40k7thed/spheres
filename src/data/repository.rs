//! The repository is a file system based structure of data files. Any changes to program data get
//! persisted here. The updater pulls in the latest changes whenever required.

use config::Config;
use std::fs::Path;

pub struct Repository {
	config: Config,
	repo: Path
}

impl Repository {

	fn new(config: Config) -> Repository {
		return Repository {
			config: config,
			repo: Path::new(env::home_dir())
				.join(".spheres")
		}
	}

	/// Saves a record to the repository, triggering a watcher update.
	pub fn write(&self) {
	}

}
