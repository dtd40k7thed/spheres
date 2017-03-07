//! The index holds all the data presented to the gui, and all read operations are directed here.
//! Whenever something changes in the repository, the change is reflected in the index.

use notify::{ RecommendedWatcher, Watcher, RecursiveMode };
use std::path::Path;
use std::time::Duration;
use std::sync::mpsc::channel;

pub struct Index {
	config: Config
}

impl Index {

	pub fn new(config: Config) -> Index {
		return Index {
			config: config
		};
	}

	/// Grabs a record from the index.
	pub fn fetch(&self) {
	}

}

fn watcher() -> notify::Result<()> {
	let (tx, rx) = channel();
	let watch_delay = Duration::from_secs(2);
	let mut watcher: RecommendedWatcher = Watcher::new(tx, watch_delay)?;
	watcher.watch(repo.to_string(), RecursiveMode::Recursive);
	loop {
		match rx.recv() {
			Err(error) => println!("watch error: {:?}", error),
			Ok(event) => match event {
				DebouncedEvent::NoticeWrite(path) => None,
				DebouncedEvent::NoticeRemove(path) => None,
				DebouncedEvent::Create(path) => readyaml(path),
				DebouncedEvent::Write(path) => readyaml(path),
				DebouncedEvent::Chmod(path) => readyaml(path),
				DebouncedEvent::Remove(path) => readyaml(path),
				DebouncedEvent::Rename(from, dest) => {
					readyaml(from);
					readyaml(dest);
				},
				DebouncedEvent::Rescan() => None,
				DebouncedEvent::Error(error, path) => None
			}
		}
	}
}
