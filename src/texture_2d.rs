use std::path::Path;
use std::os::raw::c_void;

extern crate image;
use image::{ GenericImage, GenericImageView };

pub struct Texture2D {
    pub width: u32,
    pub height: u32,
    id: u32
}

impl Texture2D {
    pub fn from_file(path: &str) -> Result<Texture2D, &str> {
        let mut id: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }

        // let img = image::open(&Path::new(path)).expect("Can't open image.");
        let img = image::open(&Path::new(path)).expect("Cannot open image.");
        let dimensions = img.dimensions();
        let data = img.to_bytes();

        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                dimensions.0 as i32, // width
                dimensions.1 as i32, // height
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                &data[0] as *const u8 as *const c_void);
        }

        Ok(Texture2D{
            width: dimensions.0,
            height: dimensions.1,
            id
        })
    }
}