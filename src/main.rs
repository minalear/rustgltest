mod shader;
mod buffer;
mod vertex_array;
mod window;
mod matrix;
mod vector;
mod texture_2d;

use shader::Shader;
use buffer::Buffer;
use vertex_array::VertexArray;
use window::Window;
use matrix::Mat4;
use vector::Vec3;
use texture_2d::Texture2D;

extern crate sdl2;
extern crate gl;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const V_SHADER_SOURCE: &str = r#"
    #version 400 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aCol;
    layout (location = 2) in vec2 aUV;

    out vec3 vColor;
    out vec2 vUV;

    void main() {
        vColor = aCol;
        vUV = aUV;
        gl_Position = vec4(aPos, 1.0);
    }
"#;

const F_SHADER_SOURCE: &str = r#"
    #version 400 core
    out vec4 FragColor;

    in vec3 vColor;
    in vec2 vUV;

    uniform sampler2D img;

    void main() {
        FragColor = texture(img, vUV) * vec4(vColor, 1.0f);
    }
"#;

fn main() {
    let window = Window::create("New Window", 1280, 720).unwrap();
    let program = Shader::create(V_SHADER_SOURCE, F_SHADER_SOURCE);

    // VBO
    let vertices: [f32; 32] = [
         0.5,  0.5, 0.0,    1.0, 1.0, 1.0,    1.0, 1.0,
         0.5, -0.5, 0.0,    1.0, 1.0, 1.0,    1.0, 0.0,
        -0.5, -0.5, 0.0,    1.0, 1.0, 1.0,    0.0, 0.0,
        -0.5,  0.5, 0.0,    1.0, 1.0, 1.0,    0.0, 1.0,
    ];
    let indices: [u32; 6] = [
        0, 1, 3,
        1, 2, 3
    ];

    let vao = VertexArray::create();
    vao.bind();

    let vbo = Buffer::create(&vertices, vertices.len(), buffer::BufferType::Vertex);
    let ebo = Buffer::create(&indices, indices.len(), buffer::BufferType::Index);

    vbo.bind();
    ebo.bind();

    vao.vertex_attrib_pointer(0, 3, 8, 0); // pos
    vao.vertex_attrib_pointer(1, 3, 8, 3); // color
    vao.vertex_attrib_pointer(2, 2, 8, 6); // uv
    vao.enable_attribute(0);
    vao.enable_attribute(1);
    vao.enable_attribute(2);

    Buffer::clear_bind(buffer::BufferType::Vertex);
    //Buffer::clear_bind(buffer::BufferType::Index);
    VertexArray::clear_bind();

    // Texture
    let image = match Texture2D::from_file("./target/debug/content/Carl.png") {
        Ok(texture) => texture,
        Err(error) => panic!("{}", error)
    };

    println!("image: {}x{}", image.width, image.height);

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
            // gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
        VertexArray::clear_bind();

        window.swap_buffers();

        //::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}