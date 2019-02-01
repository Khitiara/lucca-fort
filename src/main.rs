#[macro_use]
extern crate glium;
extern crate image;

mod assets;
mod vertex;
mod world;
mod aabb;

fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new();
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let prog = glium::Program::from_source(&display, include_str!("../assets/sprite.vert"), include_str!("../assets/sprite.frag"), None);

    let mut closed = false;
    while !closed {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });
    }
}