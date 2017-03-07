//! This module handles managing all the data needed for the application to run. All read/write
//! operations should go through here.

//mod config;
//mod index;
pub mod reader;
//mod repository;
//mod updater;

//use config::Config;
//use index::Index;
//use repository::Repository;
//use updater::Updater;

pub struct Data {
//	config:     Config,
//	index:      Index,
//	repository: Repository,
//	updater:    Updater
}

impl Data {

	pub fn new() -> Data {
//		let config = Config::new();
		return Data {
//			config:     config,
//			index:      Repository::new(config),
//			repository: Repository::new(config),
//			updater:    Updater::new(config)
		}
	}

	pub fn config_u32(path: &'static str) -> Option<u32> {
		return None;
	}

	pub fn config_str(path: &'static str) -> Option<&'static str> {
		return None;
	}

}
