//! Provides an 'EventLoop' class for managing the gui worker.
//!
use std::time::{ Instant };

/// In most of the examples the `glutin` crate is used for providing the window context and
/// events while the `glium` crate is used for displaying `conrod::render::Primitives` to the
/// screen.
///
/// This `Iterator`-like type simplifies some of the boilerplate involved in setting up a
/// glutin+glium event loop that works efficiently with conrod.
pub struct EventLoop {
	ui_needs_update: bool,
	last_update: Instant,
}

#[cfg(feature="glium")]
mod _glium;

#[cfg(feature="piston")]
mod _piston;

#[cfg(not(any(feature="glium", feature="piston")))]
impl EventLoop {
	pub fn new() -> Self { panic!("Either 'glium' or 'piston' feature required.") }
}
