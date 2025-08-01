mod snake;

use glium::Display;
use glium::backend::glutin::SimpleWindowBuilder;
use glium::glutin::surface::WindowSurface;
use glium::winit::application::ApplicationHandler;
use glium::winit::event::WindowEvent;
use glium::winit::event_loop::{ActiveEventLoop, EventLoop};
use glium::winit::window::{Window, WindowId};
use crate::snake::Snake;

pub trait App {
    fn new(display: Display<WindowSurface>) -> Self;
    fn update(&mut self);
    fn render(&mut self);
}

struct Engine<T: App> {
    window: Option<Window>,
    app: Option<T>,
}

impl<T: App> Engine<T> {
    fn new() -> Self {
        Self {
            window: None,
            app: None
        }
    }
}

impl<T: App> ApplicationHandler for Engine<T> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let (window, display) = SimpleWindowBuilder::new().build(event_loop);
        self.window = Some(window);
        self.app = Some(T::new(display));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.app.as_mut().unwrap().update();
                self.app.as_mut().unwrap().render();

                self.window.as_ref().unwrap().request_redraw();
            },
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().expect("event loop building");
    let _ = event_loop.run_app(&mut Engine::<Snake>::new());
}
