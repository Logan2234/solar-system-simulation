use glfw::Context;
use solarsys::{create_window, process_input};

extern crate gl;

fn main() {
    // Initialize glfw
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Configuration of glfw
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = create_window(&mut glfw);

    // Render loop
    while !window.should_close() {
        process_input(&mut window);

        unsafe {
            gl::ClearColor(0.15, 0.15, 0.15, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT)
        };

        window.swap_buffers();
        glfw.poll_events();
    }
}
