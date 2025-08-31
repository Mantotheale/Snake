use std::default::Default;
use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

const ONE_SEC_TIME: Duration = Duration::new(1, 0);
const UPDATE_TIME: Duration = Duration::new(0, 16_666_667); // 60 FPS

struct AppState {
    window: Window,
    update_count: u32,
    render_count: u32
}

impl AppState {
    fn update(&mut self) {
        self.update_count += 1;
    }

    fn render(&mut self) {
        self.render_count += 1;
        self.window.request_redraw();
    }

    fn one_sec_update(&mut self) {
        println!("one sec update");
        println!("UPS: {}", self.update_count);
        println!("FPS: {}", self.render_count);
        self.update_count = 0;
        self.render_count = 0;
        println!("Time: {:?}", Instant::now());
    }
}

#[derive(Default)]
struct App {
    default_window_attributes: WindowAttributes,
    state: Option<AppState>,
    next_update_time: Option<Instant>,
    next_one_sec_time: Option<Instant>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(self.default_window_attributes.clone()).unwrap();

        self.state = Some(AppState {
            window,
            update_count: 0,
            render_count: 0,
        });

        let current_time = Instant::now();
        self.next_update_time = Some(current_time + UPDATE_TIME);
        self.next_one_sec_time = Some(current_time + ONE_SEC_TIME);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.state.as_mut().unwrap().render();
            }
            _ => ()
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        let current_time = Instant::now();

        while current_time >= self.next_update_time.unwrap() {
            self.state.as_mut().unwrap().update();
            self.next_update_time = Some(self.next_update_time.unwrap() + UPDATE_TIME);
        }

        self.state.as_mut().unwrap().render();

        while current_time >= self.next_one_sec_time.unwrap() {
            self.state.as_mut().unwrap().one_sec_update();
            self.next_one_sec_time = Some(self.next_one_sec_time.unwrap() + ONE_SEC_TIME);
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let mut app = App {
        default_window_attributes: WindowAttributes::default()
            .with_title("Snake")
            .with_inner_size(PhysicalSize::new(1280, 720)),
        ..Default::default()
    };

    event_loop.run_app(&mut app).unwrap();
}