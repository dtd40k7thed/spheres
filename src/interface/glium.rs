//! Conrod interface with a winint glium backend.

use conrod;
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::{ DisplayBuild, Surface };
use ::util::EventLoop;
use ::data::Data;

impl Interface {

	pub fn new(&data: Data) -> Interface {
		return Interface {
			data: data
		}
	}

	pub fn show(&self) {
		use glium::glutin::{ Event, VirtualKeyCode };
		use conrod::backend::glium::{ Renderer };
		use conrod::image::{ Map as ImageMap };
		use conrod::backend::winit::convert;

		use glium::texture::{ Texture2d };
		use glium::glutin::WindowBuilder;

		let window_width  = self.data.config_u32("interface.window.width").or(DEFAULT_WIDTH);
		let window_height = self.data.config_u32("interface.window.height").or(DEFAULT_HEIGHT);
		let window_title  = self.data.config_str("interface.window.title").or(DEFAULT_TITLE);

		// Build the window.
		let display = WindowBuilder::new()
			.with_vsync()
			.with_dimensions(window_width, window_height)
			.with_title(window_title)
			.build_glium()
			.unwrap();

		// construct our `Ui`.
		let mut ui = conrod::UiBuilder::new([window_width as f64, window_height as f64]).build();

		//		// Add a `Font` to the `Ui`'s `font::Map` from file.
		//		let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
		//		let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
		//		ui.fonts.insert_from_file(font_path).unwrap();

		// A type used for converting `conrod::render::Primitives` into `Command`s that can be used
		// for drawing to the glium `Surface`.
		let mut renderer = Renderer::new(&display).unwrap();

		// The image map describing each of our widget->image mappings (in our case, none).
		let image_map = ImageMap::<glium::texture::Texture2d>::new();

		let mut event_loop = EventLoop::new();
		'main: loop {

			for event in event_loop.next(&display) {

				// Use the `winit` backend feature to convert the winit event to a conrod one.
				if let Some(event) = convert(event.clone(), &display) {
					ui.handle_event(event);
					event_loop.needs_update();
				}

				match event {
					// Break from the loop upon `Escape`.
					Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) |
					Event::Closed =>
						break 'main,
					_ => {},
				}

			}

			// Instantiate all widgets in the GUI.
			//			set_widgets(ui.set_widgets(), ids);

			// Render the `Ui` and then display it on the screen.
			if let Some(primitives) = ui.draw_if_changed() {
				renderer.fill(&display, primitives, &image_map);
				let mut target = display.draw();
				target.clear_color(0.0, 0.0, 0.0, 1.0);
				renderer.draw(&display, &mut target, &image_map).unwrap();
				target.finish().unwrap();
			}

		}
	}

	/*
        // Button matrix dimensions.
        const ROWS: usize = 10;
        const COLS: usize = 24;

        // Draw the Ui.
        fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids) {
            use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};
            // Construct our main `Canvas` tree.
            widget::Canvas::new().flow_down(&[
                (ids.header, widget::Canvas::new().color(color::BLUE).pad_bottom(20.0)),
                (ids.body, widget::Canvas::new().length(300.0).flow_right(&[
                    (ids.left_column, widget::Canvas::new().color(color::LIGHT_ORANGE).pad(20.0)),
                    (ids.middle_column, widget::Canvas::new().color(color::ORANGE)),
                    (ids.right_column, widget::Canvas::new().color(color::DARK_ORANGE).pad(20.0)),
                ])),
                (ids.footer, widget::Canvas::new().color(color::BLUE).scroll_kids_vertically()),
            ]).set(ids.master, ui);

            // A scrollbar for the `FOOTER` canvas.
            widget::Scrollbar::y_axis(ids.footer).auto_hide(true).set(ids.footer_scrollbar, ui);

            // Now we'll make a couple floating `Canvas`ses.
            let floating = widget::Canvas::new().floating(true).w_h(110.0, 150.0).label_color(color::WHITE);
            floating.middle_of(ids.left_column).title_bar("Blue").color(color::BLUE).set(ids.floating_a, ui);
            floating.middle_of(ids.right_column).title_bar("Orange").color(color::LIGHT_ORANGE).set(ids.floating_b, ui);

            // Here we make some canvas `Tabs` in the middle column.
            widget::Tabs::new(&[(ids.tab_foo, "FOO"), (ids.tab_bar, "BAR"), (ids.tab_baz, "BAZ")])
                .wh_of(ids.middle_column)
                .color(color::BLUE)
                .label_color(color::WHITE)
                .middle_of(ids.middle_column)
                .set(ids.tabs, ui);

            widget::Text::new("Fancy Title")
                .color(color::LIGHT_ORANGE)
                .font_size(48)
                .middle_of(ids.header)
                .set(ids.title, ui);
            widget::Text::new("Subtitle")
                .color(color::BLUE.complement())
                .mid_bottom_of(ids.header)
                .set(ids.subtitle, ui);

            widget::Text::new("Top Left")
                .color(color::LIGHT_ORANGE.complement())
                .top_left_of(ids.left_column)
                .set(ids.top_left, ui);

            widget::Text::new("Bottom Right")
                .color(color::DARK_ORANGE.complement())
                .bottom_right_of(ids.right_column)
                .set(ids.bottom_right, ui);

            fn text (text: widget::Text) -> widget::Text { text.color(color::WHITE).font_size(36) }
            text(widget::Text::new("Foo!")).middle_of(ids.tab_foo).set(ids.foo_label, ui);
            text(widget::Text::new("Bar!")).middle_of(ids.tab_bar).set(ids.bar_label, ui);
            text(widget::Text::new("BAZ!")).middle_of(ids.tab_baz).set(ids.baz_label, ui);

            let footer_wh = ui.wh_of(ids.footer).unwrap();
            let mut elements = widget::Matrix::new(COLS, ROWS)
                .w_h(footer_wh[0], footer_wh[1] * 2.0)
                .mid_top_of(ids.footer)
                .set(ids.button_matrix, ui);
            while let Some(elem) = elements.next(ui) {
                let (r, c) = (elem.row, elem.col);
                let n = c + r * c;
                let luminance = n as f32 / (COLS * ROWS) as f32;
                let button = widget::Button::new().color(color::BLUE.with_luminance(luminance));
                for _click in elem.set(button, ui) {
                    println!("Hey! {:?}", (r, c));
                }
            }

            let button = widget::Button::new().color(color::RED).w_h(30.0, 30.0);
            for _click in button.clone().middle_of(ids.floating_a).set(ids.bing, ui) {
                println!("Bing!");
            }
            for _click in button.middle_of(ids.floating_b).set(ids.bong, ui) {
                println!("Bong!");
            }
        }
    */

}
