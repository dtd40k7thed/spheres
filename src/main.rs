//! Wires up the application.

#![feature(associated_consts)]
#![allow(warnings)]

#[macro_use] extern crate conrod;
             extern crate yaml_rust;
             extern crate notify;

mod data;
mod interface;
mod util;

pub fn main() {
	let data = data::Data::new();
	interface::Interface::new(&data)
		.show();
}
