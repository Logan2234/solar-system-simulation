extern crate gl;

use std::sync::mpsc::Receiver;

use glfw::Context;
use glfw::Glfw;
use glfw::Window;
use glfw::WindowEvent;

use crate::{WINDOW_WIDTH, WINDOW_HEIGHT};

/// Wrapper function to create a window
pub fn create_window(glfw: &mut Glfw) -> (Window, Receiver<(f64, WindowEvent)>) {
    let (mut window, events) = glfw.create_window(
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            "window",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    // Makes the context of the specified window current
    window.make_current();

    // Loads the function pointers into their respective function pointers
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    // Specifies the rendering window dimensions to OpenGL
    unsafe {
        gl::Viewport(
            0,
            0,
            WINDOW_WIDTH.try_into().unwrap(),
            WINDOW_HEIGHT.try_into().unwrap(),
        );
    }
    (window, events)
}

/// Function called in the render loop to check user's input.
///
/// `window`: the window to check for input
pub fn process_input(window: &mut Window) {
    if window.get_key(glfw::Key::Escape) == glfw::Action::Press {
        window.set_should_close(true);
    }
}
