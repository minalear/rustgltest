use sdl2::video::GLProfile;

pub struct Window {
    pub sdl_context: sdl2::Sdl,
    pub sdl_window: sdl2::video::Window,
    pub width: u32,
    pub height: u32,
    _gl_context: sdl2::video::GLContext
}

impl Window {
    pub fn create(title: &str, width: u32, height: u32) -> Result<Window, String> {
        let sdl_context = sdl2::init()?;
        let video = sdl_context.video()?;

        video.gl_attr().set_context_profile(GLProfile::Core);
        video.gl_attr().set_context_version(4, 0);

        let window = match video.window(title, width, height)
            .opengl()
            .position_centered()
            .build()
        {
            Ok(window) => window,
            Err(err) => return Err(format!("SDL Init Error: {}", err))
        };

        let gl_context = window.gl_create_context()?;
        gl::load_with(|name| video.gl_get_proc_address(name) as *const _);

        if video.gl_attr().context_profile() != GLProfile::Core {
            return Err(String::from("SDL Init Error: Invalid GL Profile"));
        }
        if video.gl_attr().context_version() != (4, 0) {
            return Err(String::from("SDL Init Error: Invalid GL Version"));
        }

        Ok(Window{
            sdl_context,
            sdl_window: window,
            width,
            height,
            _gl_context: gl_context
        })
    }

    pub fn swap_buffers(&self) {
        self.sdl_window.gl_swap_window();
    }
}