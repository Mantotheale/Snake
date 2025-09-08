mod entry_point;
mod input_manager;

use glium::IndexBuffer;
use std::rc::Rc;
use std::time::{Duration, Instant};
use glium::{implement_vertex, uniform, winit, Display, Program, Surface, Texture2d, VertexBuffer};
use glium::glutin::surface::WindowSurface;
use glium::index::PrimitiveType::TrianglesList;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowAttributes};
use crate::entry_point::{App, EntryPoint};
use crate::input_manager::InputManager;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2]
}
implement_vertex!(Vertex, position, tex_coords);

#[derive(Clone)]
struct Quad {
    vertices: [Vertex; 4],
    texture: Rc<Texture2d>
}

impl Quad {
    pub fn new(texture: Rc<Texture2d>) -> Self {
        Self {
            vertices: [
                Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
                Vertex { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0] },
                Vertex { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] },
                Vertex { position: [ -1.0,  1.0], tex_coords: [0.0, 1.0] }
            ],
            texture
        }
    }

    pub fn indices<'a>() -> &'a[u8] {
        &[0, 1, 3, 1, 2, 3]
    }
}

struct Snake {
    window: Window,
    display: Display<WindowSurface>,
    vertex_buffer: VertexBuffer<Vertex>,
    quad: Quad,
    indices: IndexBuffer<u8>,
    program: Program,
    update_count: u32,
    render_count: u32,
    should_close: bool,
    input_manager: InputManager,
}

impl App for Snake {
    fn new(window: Window, display: Display<WindowSurface>) -> Self {
        let image = image::load(std::io::Cursor::new(&include_bytes!("../assets/images/dices.png")),
                                image::ImageFormat::Png).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = Rc::new(Texture2d::new(&display, image).unwrap());

        let quad = Quad::new(texture);
        let vertex_buffer = VertexBuffer::new(&display, &quad.vertices).unwrap();
        let indices = IndexBuffer::new(&display, TrianglesList, Quad::indices()).unwrap();

        let vertex_shader_src = r#"
            #version 330

            in vec2 position;
            in vec2 tex_coords;

            out vec2 v_tex_coords;

            void main() {
                v_tex_coords = tex_coords;
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;


        let fragment_shader_src = r#"
            #version 330

            in vec2 v_tex_coords;

            out vec4 color;

            uniform sampler2D tex;

            void main() {
                color = texture(tex, v_tex_coords);
            }
        "#;

        let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

        Self {
            window,
            display,
            vertex_buffer,
            quad,
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
            &uniform! {
                tex: &*self.quad.texture
            },
            &glium::DrawParameters {
                blend: glium::Blend::alpha_blending(),
                ..Default::default()
            }
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