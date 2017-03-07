//! Entry point for interface creation and management.

use ::data::Data;

const DEFAULT_WIDTH:  u32  = 800;
const DEFAULT_HEIGHT: u32  = 600;
const DEFAULT_TITLE: &'static str = "Spheres: DtD 40k 7th edition Character Editor";

pub struct Interface {
	data: Data,
}

#[cfg(all(feature="winit", feature="glium"))]
mod glium;


#[cfg(not(all(feature="winit", feature="glium")))]
impl Interface {
	pub fn new(data: &Data) -> Self { panic!("You need one of the available backends enabled.") }
	pub fn show(&self)              { panic!("You need one of the available backends enabled.") }
}
