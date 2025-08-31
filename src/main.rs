mod entry_point;

use std::default::Default;
use std::time::{Duration, Instant};
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowAttributes};
use crate::entry_point::{App, EntryPoint};

struct Snake {
    window: Window,
    update_count: u32,
    render_count: u32,
    should_close: bool
}

impl App for Snake {
    fn new(window: Window) -> Self {
        Self {
            window,
            update_count: 0,
            render_count: 0,
            should_close: false
        }
    }

    fn process_input(&mut self, input: WindowEvent) {
        if let WindowEvent::CloseRequested = input {
            self.should_close = true;
        }

        println!("{:?}", input);
    }

    fn update(&mut self) {
        self.update_count += 1;
    }

    fn one_sec_update(&mut self) {
        println!("one sec update");
        println!("UPS: {}", self.update_count);
        println!("FPS: {}", self.render_count);
        self.update_count = 0;
        self.render_count = 0;
        println!("Time: {:?}", Instant::now());
    }

    fn render(&mut self) {
        self.render_count += 1;
    }

    fn should_close(&self) -> bool {
        self.should_close
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window_attributes = WindowAttributes::default()
        .with_title("Snake")
        .with_inner_size(PhysicalSize::new(1280, 720));

    let wake_up = event_loop.create_proxy();

    let mut entry_point = EntryPoint::<Snake>::new(
        window_attributes, Duration::from_secs_f64(1f64 / 60f64),
        wake_up
    );

    event_loop.run_app(&mut entry_point).unwrap();
}