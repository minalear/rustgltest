pub struct VertexBuffer {
    id: u32
}

impl VertexBuffer {
    pub fn create(vertices: &[f32], len: usize) -> VertexBuffer {
        use std::{ mem };
        use std::os::raw::c_void;

        let mut vbo: u32 = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (len * mem::size_of::<f32>()) as isize,
                           &vertices[0] as *const f32 as *const c_void,
                           gl::STATIC_DRAW
            );
        }

        VertexBuffer{
            id: vbo
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn clear_bind() {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}