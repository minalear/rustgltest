use std::{ ffi::CString, ptr, str };
use gl::types::*;

pub struct Shader {
    pub program_id: u32
}

fn check_shader_comp_status(shader: GLuint) -> bool {
    let mut success = gl::FALSE as GLint;
    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1); // Account for null character
            gl::GetShaderInfoLog(shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            println!("ERROR::SHADER::COMPILE_ERROR\n{}", str::from_utf8(&info_log).unwrap());
            info_log.set_len(0); // clear?

            return false;
        }
    }

    true
}
fn check_shader_link_status(program: GLuint) -> bool {
    let mut success = gl::FALSE as GLint;
    unsafe {
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1); // Account for null character
            gl::GetProgramInfoLog(program, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            println!("ERROR::SHADER::LINK_ERROR\n{}", str::from_utf8(&info_log).unwrap());
            info_log.set_len(0); // clear?

            return false;
        }
    }

    true
}

fn create_shader(source: &str, shader_type: GLenum) -> GLuint {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        let c_str = CString::new(source.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        check_shader_comp_status(shader);

        shader
    }
}

impl Shader {
    pub fn create(v_source: &str, f_source: &str) -> Shader {
        // Shaders
        let v_shader = create_shader(v_source, gl::VERTEX_SHADER);
        let f_shader = create_shader(f_source, gl::FRAGMENT_SHADER);

        // Link Shaders
        let program = unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, v_shader);
            gl::AttachShader(program, f_shader);
            gl::LinkProgram(program);

            program
        };

        check_shader_link_status(program);

        // Cleanup
        unsafe {
            gl::DeleteShader(v_shader);
            gl::DeleteShader(f_shader);
        }

        Shader{
            program_id: program
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }
}