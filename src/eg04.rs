extern crate sdl2;

use sdl2::video::{Window, PosCentered, OPENGL};
use sdl2::event::{Quit, poll_event};
use sdl2::surface::{Surface};

fn main() {
    sdl2::init(sdl2::INIT_EVERYTHING);

    let window  = match Window::new("eg04", PosCentered, PosCentered, 640, 480, OPENGL) {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };
    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    // Load a surface.
    // Surfaces live in system RAM, so they aren't ideal for performance.
    let surface = match Surface::from_bmp(&Path::new("res/ice-troll.bmp")) {
        Ok(surface) => surface,
        Err(err)    => panic!("failed to load surface: {}", err)
    };

    // Convert a surface to a texture.
    // Textures can be used more efficiently by the GPU. (If one is available.)
    let texture = match renderer.create_texture_from_surface(&surface) {
        Ok(texture) => texture,
        Err(err)    => panic!("failed to convert surface: {}", err)
    };

    let _ = renderer.clear();
    // Display the texture.
    // Omitting the src & dst Rect arguments will cause our image to stretch across the entire buffer.
    // Try passing Some(surface.get_rect()) for src & dst instead of None & see how things change.
    let _ = renderer.copy(&texture, None, None);
    let _ = renderer.present();


    // loop until we receive a QuitEvent
    'event : loop {
        match poll_event() {
            Quit(_) => break 'event,
            _            => continue
        }
    }

    sdl2::quit();
}

