//! Reads yaml files, turns them into data structs, and back again.

use yaml_rust::yaml::{ Yaml, YamlLoader };
use std::path::{ Path };
use std::fs::{ File };
use std::io::{ Read };
use std::ops::{ Deref };
use ::util::AppError;

pub fn write(path: &'static str) -> Result<(), AppError> {
	return Ok(())
}

pub fn read(path: &'static str) -> Result<(), AppError> {
	let filepath = Path::new(path);
	if filepath.exists() {

		let mut filecontent = String::new();
		File::open(path)?
			.read_to_string(&mut filecontent)?;

		let docs = YamlLoader::load_from_str(filecontent.deref())?;
		for doc in &docs {
			// convert document into appropriate data structure.

//			match doc {
//				Yaml::Real =>;
//				Yaml::Integer =>;
//				Yaml::String =>;
//				Yaml::Boolean =>;
//				Yaml::Array =>;
//				Yaml::Hash =>;
//				Yaml::Alias =>;
//				Yaml::Null =>;
//				Yaml::BadValue =>;
//			}

			// Create / Update structure in db.
		}
	} else {
		// Remove document from index.
	}

	Ok(())

}
