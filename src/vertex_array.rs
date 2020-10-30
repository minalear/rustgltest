use std::mem;
use std::os::raw::c_void;
use gl::types::*;

pub struct VertexArray {
    id: u32
}

impl VertexArray {
    pub fn create() -> VertexArray {
        let mut vao: u32 = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }

        VertexArray {
            id: vao
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn vertex_attrib_pointer(&self, index: u32, size: i32, stride: i32, offset: i32) {
        unsafe {
            gl::VertexAttribPointer(
                index,
                size,
                gl::FLOAT,
                gl::FALSE,
                stride * mem::size_of::<f32>() as GLsizei,
                (offset as usize * mem::size_of::<f32>()) as *const c_void
            );
        }
    }
    pub fn enable_attribute(&self, id: u32) {
        unsafe {
            gl::EnableVertexAttribArray(id);
        }
    }
    pub fn disable_attribute(&self, id: u32) {
        unsafe {
            gl::DisableVertexAttribArray(id);
        }
    }

    pub fn clear_bind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}