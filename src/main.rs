mod entry_point;
mod input_manager;

use std::time::{Duration, Instant};
use glium::{implement_vertex, winit, Display, Program, Surface, VertexBuffer};
use glium::glutin::surface::WindowSurface;
use glium::index::NoIndices;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowAttributes};
use crate::entry_point::{App, EntryPoint};
use crate::input_manager::InputManager;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

struct Snake {
    window: Window,
    display: Display<WindowSurface>,
    vertex_buffer: VertexBuffer<Vertex>,
    indices: NoIndices,
    program: Program,
    update_count: u32,
    render_count: u32,
    should_close: bool,
    input_manager: InputManager,
}

impl App for Snake {
    fn new(window: Window, display: Display<WindowSurface>) -> Self {
        let vertex1 = Vertex { position: [-0.5, -0.5] };
        let vertex2 = Vertex { position: [ 0.0,  0.5] };
        let vertex3 = Vertex { position: [ 0.5, -0.25] };
        let shape = vec![vertex1, vertex2, vertex3];
        let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let vertex_shader_src = r#"
            #version 330

            in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;


        let fragment_shader_src = r#"
            #version 330

            out vec4 color;

            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;

        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

        Self {
            window,
            display,
            vertex_buffer,
            indices,
            program,
            update_count: 0,
            render_count: 0,
            should_close: false,
            input_manager: InputManager::new()
        }
    }

    fn process_input(&mut self, input: WindowEvent) {
        if let WindowEvent::RedrawRequested = &input {
            self.render();
        }

        if let WindowEvent::CloseRequested = input {
            self.should_close = true;
        }

        self.input_manager.receive_input(input);
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
        println!("Mouse: {:?}", self.input_manager.cursor_position());
    }

    fn render(&mut self) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(
            &self.vertex_buffer,
            &self.indices,
            &self.program,
            &glium::uniforms::EmptyUniforms,
            &Default::default()
        ).unwrap();
        target.finish().unwrap();

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

    let update_time = 1f64 / 60f64;
    let mut entry_point = EntryPoint::<Snake>::new(
        window_attributes, Duration::from_secs_f64(update_time),
        wake_up
    );

    event_loop.run_app(&mut entry_point).unwrap();
}