//! Base error

use yaml_rust::{ ScanError };
use std::io::Error as IoError;

#[derive(Debug)]
pub enum AppError {
	YamlScan(ScanError),
	Io(IoError),
}

impl From<ScanError> for AppError {
	fn from(error: ScanError) -> Self {
		return AppError::YamlScan(error);
	}
}

impl From<IoError> for AppError {
	fn from(error: IoError) -> Self {
		return AppError::Io(error);
	}
}
