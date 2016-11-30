// TODO: Remove these allows
#![allow(unused_imports, unused_must_use, unused_variables)]
#[macro_use]
extern crate conrod;
extern crate coa;
extern crate clap;

pub use coa::TempDir;
pub use coa::cuddle::{Answer, Archive, ArchiveInfo, Question, Team, Topic, settings as cuddle_settings};
pub use coa::error::{Result as CuddleResult, Error as CuddleError};
pub use conrod::backend::glium::glium;
use conrod::backend::glium::glium::{DisplayBuild, Surface};

mod app;
mod support;

fn main() {
    let matches = clap::App::new("Cuddly Content Creator")
        .bin_name("cuddly-content")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Victor M. Suarez <svmnotn@gmail.com>")
        .about("A content creator for the Cuddly Octo Adventure Project.")
        .args_from_usage("[file]             'A cuddle archive to load at the beginning of the program'
                          -d --data=[folder] 'A folder in which to keep all the temporary data, Defaults to .cuddles'")
        .get_matches();

    let data = if let Some(val) = matches.value_of("data") {
        val
    } else {
        std::fs::create_dir(".cuddles");
        ".cuddles"
    };

    let (archive, dir) = if let Some(loc) = matches.value_of("file") {
        coa::load_cuddle(loc, Some(data)).expect("Unable to load the given cuddle")
    } else {
        use std::time::{UNIX_EPOCH, SystemTime};

        (Archive::default(),
         TempDir::new_in(data,
                         &format!("new-cuddle-{:?}",
                                  SystemTime::now()
                                      .duration_since(UNIX_EPOCH)
                                      .expect("Now is before UNIX_EPOCH? Clock is wrong!")
                                      .as_secs()))
             .expect("Unable to create temporary folder"))
    };

    run(archive, dir, data.into());
}

/// A set of reasonable stylistic defaults that works for the `gui` below.
fn theme() -> conrod::Theme {
    conrod::Theme {
        name: "Demo Theme".to_string(),
        padding: conrod::Padding::none(),
        x_position: conrod::Position::Align(conrod::Align::Start, None),
        y_position: conrod::Position::Direction(conrod::Direction::Backwards, 20.0, None),
        background_color: conrod::color::DARK_CHARCOAL,
        shape_color: conrod::color::LIGHT_CHARCOAL,
        border_color: conrod::color::DARK_BLUE,
        border_width: 0.0,
        label_color: conrod::color::BLACK,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: std::collections::HashMap::new(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}

fn run(archive: Archive, dir: TempDir, data: String) {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;

    // Build the window.
    let display = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("Cuddly Content Creator")
        .build_glium()
        .expect("Unable to build the window");

    // Construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).theme(theme()).build();
    // Hack for the font loading, since it is required.
    // TODO: properly load font
    ui.fonts.insert_from_file("Vera.ttf").expect("Unable to find font");

    // Make the editor state
    let mut editor = app::ArchiveEditor::new(archive,
                                             data,
                                             dir,
                                             support::Ids::new(ui.widget_id_generator()),
                                             conrod::image::Map::new());

    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    //
    // Internally, the `Renderer` maintains:
    // - a `backend::glium::GlyphCache` for caching text onto a `glium::texture::Texture2d`.
    // - a `glium::Program` to use as the shader program when drawing to the `glium::Surface`.
    // - a `Vec` for collecting `backend::glium::Vertex`s generated when translating the
    // `conrod::render::Primitive`s.
    // - a `Vec` of commands that describe how to draw the vertices.
    let mut renderer = conrod::backend::glium::Renderer::new(&display).expect("Unable to optain renderer");

    // Start the loop:
    //
    // - Render the current state of the `Ui`.
    // - Update the widgets.
    // - Poll the window for available events.
    // - Repeat.
    'main: loop {

        // Poll for events.
        for event in display.poll_events() {

            // Use the `glutin` backend feature to convert the glutin event to a conrod one.
            let window = display.get_window().expect("Unable to get the window");
            if let Some(event) = conrod::backend::glutin::convert(event.clone(), window) {
                ui.handle_event(event);
            }

            match event {
                // Break from the loop upon `Escape`.
                glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape)) |
                glium::glutin::Event::Closed => break 'main,
                _ => {}
            }
        }


            // We must manually track the window width and height as it is currently not possible to
            // receive `Resize` events from glium on Mac OS any other way.
            //
            // TODO: Once the following PR lands, we should stop tracking size like this and use the
            // `window_resize_callback`. https://github.com/tomaka/winit/pull/88
            if let Some(win_rect) = ui.rect_of(ui.window) {
                let (win_w, win_h) = (win_rect.w() as u32, win_rect.h() as u32);
                let (w, h) = display.get_window().unwrap().get_inner_size_points().unwrap();
                if w != win_w || h != win_h {
                    let event = conrod::event::Input::Resize(w, h);
                    ui.handle_event(event);
                }
            }

        // If some input event has been received, update the GUI.
        if ui.global_input.events().next().is_some() {
            // Instantiate a GUI demonstrating every widget type provided by conrod.
            let mut ui = ui.set_widgets();
            editor.gui(&mut ui);
        }

        // Draw the `Ui`.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &editor.img_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &editor.img_map).expect("Renderer unable to draw");
            target.finish().expect("Display unable to finsh drawing");
        }

        // Avoid hogging the CPU.
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
