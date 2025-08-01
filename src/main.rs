use glium::{Display, Surface};
use glium::backend::glutin::SimpleWindowBuilder;
use glium::glutin::surface::WindowSurface;
use glium::winit::application::ApplicationHandler;
use glium::winit::event::WindowEvent;
use glium::winit::event_loop::{ActiveEventLoop, EventLoop};
use glium::winit::window::{Window, WindowId};

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

struct Snake {
    display: Display<WindowSurface>
}

impl App for Snake {
    fn new(display: Display<WindowSurface>) -> Self {
        Self {
            display
        }
    }

    fn update(&mut self) {
    }

    fn render(&mut self) {
        let mut target = self.display.draw();

        target.clear_color(0.0, 0.0, 0.0, 1.0);

        target.finish().unwrap();
    }
}

fn main() {
    let event_loop = EventLoop::new().expect("event loop building");
    let _ = event_loop.run_app(&mut Engine::<Snake>::new());
}
