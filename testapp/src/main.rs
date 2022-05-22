extern crate glfw;

use std::borrow::BorrowMut;
use glfw::{Action, Key};
use katengine::kat;
use katengine::kat::{Buffer, BufferTarget, DrawMode, VertexArray};

fn handle_events(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}

fn main() {
    let mut engine = kat::Engine::create();
    let mut window = engine.new_window(800, 800, "Hello!");

    let col = kat::Color::create(209.0 / 255.0, 159.0 / 255.0, 42.0 / 255.0, 1.0);

    let mut vertex_buffer = Buffer::<f32>::create(BufferTarget::Array, vec!(0.0, 0.0, 1.0, 1.0, 0.0, 1.0));
    let mut vertex_array = VertexArray::new();
    vertex_array.vertex_buffer(&mut vertex_buffer, vec!(2));

    while window.is_open() {
        window.update_events(engine.borrow_mut(), handle_events);

        engine.clear(&col);
        vertex_array.draw_arrays(DrawMode::Triangles, 9, 0);

        window.swap();
    }

    println!("Goodbye!")
}
