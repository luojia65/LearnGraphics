// mod teapot;

use glium::glutin;
use glium::Surface;
use glium::{implement_vertex, uniform};
// use std::io::Cursor;

#[derive(Copy, Clone)]
struct Vertex {
    pos: [f32; 3],
    color: [f32; 3],  
}

implement_vertex!(Vertex, pos, color);  

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(glutin::dpi::LogicalSize::new(500.0, 500.0))
        .with_title("OpenGL from Rust");
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let shape = vec![
        Vertex { pos: [0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0]},
        Vertex { pos: [-0.5, -0.5, 0.0],color: [0.0, 1.0, 0.0]},
        Vertex { pos: [0.0, 0.5, 0.0],  color: [0.0, 0.0, 1.0]},
    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    
    let vertex_shader_src = r#"
        #version 330 core
        layout(location = 0) in vec3 pos;
        layout(location = 1) in vec3 color;
        out vec3 ourColor;
        void main() {
            gl_Position = vec4(pos, 1.0);
            ourColor = color;
        }
    "#;
    let fragment_shader_src = r#"
        #version 330 core
        out vec4 color;
        in vec3 ourColor;
        void main() {
            color = vec4(ourColor, 1.0f);
        }
    "#;
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let mut closed = false;
    while !closed {
        let mut target = display.draw();
        target.clear_color(0.5, 0.6, 0.7, 1.0);
        let uniforms = uniform!{};
        target.draw(&vertex_buffer, &indices, &program, &uniforms,
            &Default::default()).unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => ()
                },
                _ => (),
            }
        });
    }
}
