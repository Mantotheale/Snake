use glium::{implement_vertex, Display, IndexBuffer, Program, Surface, VertexBuffer};
use glium::glutin::surface::WindowSurface;
use crate::App;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3]
}
implement_vertex!(Vertex, position, color);

pub struct Snake {
    display: Display<WindowSurface>,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
    shader: Program
}

impl App for Snake {
    fn new(display: Display<WindowSurface>) -> Self {
        let vertices = vec![
            Vertex { position: [-0.5, -0.5], color: [0.0, 0.0, 1.0] },
            Vertex { position: [0.5, -0.5], color: [0.0, 0.0, 1.0] },
            Vertex { position: [0.5, 0.5], color: [0.0, 0.0, 1.0] },
            Vertex { position: [-0.5, 0.5], color: [0.0, 0.0, 1.0] },
        ];
        let vertex_buffer = VertexBuffer::new(&display, &vertices).unwrap();

        let indices = vec![
            0, 1, 3,
            1, 2, 3
        ];
        let index_buffer =
            IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

        let vertex_shader_src = r#"
            #version 330 core
            layout (location = 0) in vec2 position;
            layout (location = 1) in vec3 color;

            out vec3 v_color;

            void main() {
                v_color = color;
                gl_Position = vec4(position, 0.0, 1.0);
            }
            "#;

        let fragment_shader_src = r#"
            #version 330 core

            in vec3 v_color;

            out vec4 color;

            void main() {
                color = vec4(v_color, 1.0);
            }
            "#;

        let shader = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

        Self {
            display,
            vertex_buffer,
            index_buffer,
            shader
        }
    }

    fn update(&mut self) {
    }

    fn render(&mut self) {
        //self.normals = Some(VertexBuffer::new(self.display.as_ref().unwrap(), &teapot::NORMALS).unwrap());
        //self.indices = Some(IndexBuffer::new(self.display.as_ref().unwrap(), index::PrimitiveType::TrianglesList,
                                             //&teapot::INDICES).unwrap());

        let mut target = self.display.draw();

        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(
            &self.vertex_buffer,
            &self.index_buffer,
            &self.shader,
            &glium::uniforms::EmptyUniforms,
            &Default::default()
        ).unwrap();

        target.finish().unwrap();
    }
}