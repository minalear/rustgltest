pub enum BufferType {
    Vertex,
    Index
}

pub struct Buffer {
    id: u32,
    buffer_type: BufferType
}

impl Buffer {
    pub fn create<T>(data: &[T], len: usize, buffer_type: BufferType) -> Buffer {
        use std::{ mem };
        use std::os::raw::c_void;

        let mut buffer_id: u32 = 0;
        let gl_type = match buffer_type {
            BufferType::Vertex => gl::ARRAY_BUFFER,
            BufferType::Index => gl::ELEMENT_ARRAY_BUFFER
        };

        unsafe {
            gl::GenBuffers(1, &mut buffer_id);
            gl::BindBuffer(gl_type, buffer_id);
            gl::BufferData(gl_type,
                           (len * mem::size_of::<T>()) as isize,
                           &data[0] as *const T as *const c_void,
                           gl::STATIC_DRAW
            );
        }

        Buffer {
            id: buffer_id,
            buffer_type
        }
    }

    pub fn bind(&self) {
        unsafe {
            match self.buffer_type {
                BufferType::Vertex => gl::BindBuffer(gl::ARRAY_BUFFER, self.id),
                BufferType::Index => gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id)
            }
        }
    }

    pub fn clear_bind(buffer_type: BufferType) {
        unsafe {
            match buffer_type {
                BufferType::Vertex => gl::BindBuffer(gl::ARRAY_BUFFER, 0),
                BufferType::Index => gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0)
            }
        }
    }
}