use std::time::{Duration, Instant};
use glium::backend::glutin::SimpleWindowBuilder;
use glium::{winit, Display};
use glium::glutin::surface::WindowSurface;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoopProxy};
use winit::window::{Window, WindowAttributes, WindowId};

const ONE_SEC_TIME: Duration = Duration::new(1, 0);

pub trait App {
    fn new(window: Window, display: Display<WindowSurface>) -> Self;
    fn process_input(&mut self, input: WindowEvent);
    fn update(&mut self);
    fn one_sec_update(&mut self);
    fn render(&mut self);
    fn should_close(&self) -> bool;
}

pub struct EntryPoint<T: App> {
    default_window_attributes: WindowAttributes,
    app: Option<T>,
    update_span: Duration,
    next_update_time: Option<Instant>,
    next_one_sec_time: Option<Instant>,
    wake_up: EventLoopProxy<()>
}

impl<T: App> EntryPoint<T> {
    pub fn new(window_attributes: WindowAttributes, update_span: Duration, wake_up: EventLoopProxy<()>) -> Self {
        Self {
            default_window_attributes: window_attributes,
            app: None,
            update_span,
            next_update_time: None,
            next_one_sec_time: None,
            wake_up
        }
    }
}

impl<T: App> ApplicationHandler for EntryPoint<T> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let (window, display) = SimpleWindowBuilder::new()
            .set_window_builder(self.default_window_attributes.clone())
            .build(event_loop);

        self.app = Some(T::new(window, display));

        let current_time = Instant::now();
        self.next_update_time = Some(current_time + self.update_span);
        self.next_one_sec_time = Some(current_time + ONE_SEC_TIME);
    }

    fn window_event(&mut self, _: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        self.app.as_mut().unwrap().process_input(event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.app.as_ref().unwrap().should_close() {
            event_loop.exit();
            return;
        }

        let current_time = Instant::now();

        while current_time >= self.next_update_time.unwrap() {
            self.app.as_mut().unwrap().update();
            self.next_update_time = Some(self.next_update_time.unwrap() + self.update_span);
        }

        self.app.as_mut().unwrap().render();

        while current_time >= self.next_one_sec_time.unwrap() {
            self.app.as_mut().unwrap().one_sec_update();
            self.next_one_sec_time = Some(self.next_one_sec_time.unwrap() + ONE_SEC_TIME);
        }

        self.wake_up.send_event(()).unwrap()
    }
}