//! Provides an 'EventLoop' class for managing the gui worker.

use std::thread;
use std::time::{ Instant, Duration };

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
use glium::{ Display as GliumDisplay };

#[cfg(feature="glium")]
use glium::glutin::{ Event as GliumEvent };

#[cfg(feature="glium")]
impl EventLoop {

	pub fn new() -> Self {
		EventLoop {
			last_update: Instant::now(),
			ui_needs_update: true,
		}
	}

	/// Produce an iterator yielding all available events.
	pub fn next(&mut self, display: &GliumDisplay) -> Vec<GliumEvent> {

		// We don't want to loop any faster than 60 FPS, so wait until it has been at least 16ms
		// since the last yield.
		let last_update = self.last_update;
		let sixteen_ms = Duration::from_millis(16);
		let duration_since_last_update = Instant::now().duration_since(last_update);
		if duration_since_last_update < sixteen_ms {
			thread::sleep(sixteen_ms - duration_since_last_update);
		}

		// Collect all pending events.
		let mut events = Vec::new();
		events.extend(display.poll_events());

		// If there are no events and the `Ui` does not need updating, wait for the next event.
		if events.is_empty() && !self.ui_needs_update {
			events.extend(display.wait_events().next());
		}

		self.ui_needs_update = false;
		self.last_update = Instant::now();

		return events

	}

	/// Notifies the event loop that the `Ui` requires another update whether or not there are any
	/// pending events.
	///
	/// This is primarily used on the occasion that some part of the `Ui` is still animating and
	/// requires further updates to do so.
	pub fn needs_update(&mut self) {
		self.ui_needs_update = true;
	}

}

#[cfg(not(feature="glium"))]
impl EventLoop {

	pub fn new() -> Self {
		panic!("Glium feature required.")
	}

}
