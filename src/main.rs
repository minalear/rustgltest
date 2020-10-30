mod shader;
mod vertex_buffer;
mod vertex_array;
mod window;
mod matrix;
mod vector;

use shader::Shader;
use vertex_buffer::VertexBuffer;
use vertex_array::VertexArray;
use window::Window;
use matrix::Mat4;
use vector::Vec3;

extern crate sdl2;
extern crate gl;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const V_SHADER_SOURCE: &str = r#"
    #version 400 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aCol;

    out vec3 vColor;

    void main() {
        vColor = aCol;
        gl_Position = vec4(aPos, 1.0);
    }
"#;

const F_SHADER_SOURCE: &str = r#"
    #version 400 core
    out vec4 FragColor;

    in vec3 vColor;

    void main() {
        FragColor = vec4(vColor, 1.0f);
    }
"#;

fn main() {
    let mat = Mat4::translate(-3.0, -7.0, 0.0) * Mat4::scale_uniform(2.0);
    let pos = Vec3::new(3.0, 7.0, 0.0);

    println!("mat: {:?}", mat);
    println!("mat * pos = {:?}", mat * pos);

    /*let window = Window::create("New Window", 1280, 720).unwrap();
    let program = Shader::create(V_SHADER_SOURCE, F_SHADER_SOURCE);

    // VBO
    let vertices: [f32; 18] = [
        -0.5, -0.5, 0.0,    1.0, 0.0, 1.0,
        0.5, -0.5, 0.0,     1.0, 1.0, 0.0,
        0.0, 0.5, 0.0,      0.0, 1.0, 1.0
    ];

    let buffer = VertexBuffer::create(&vertices, vertices.len());
    let vao = VertexArray::create();

    vao.bind();
    buffer.bind();

    vao.vertex_attrib_pointer(0, 3, 6, 0);
    vao.vertex_attrib_pointer(1, 3, 6, 3);
    vao.enable_attribute(0);
    vao.enable_attribute(1);

    VertexBuffer::clear_bind();
    VertexArray::clear_bind();

    let mut event_pump = window.sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        unsafe {
            gl::ClearColor(0.391, 0.582, 0.928, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        program.bind();
        vao.bind();
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        VertexArray::clear_bind();

        window.swap_buffers();

        //::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }*/
}