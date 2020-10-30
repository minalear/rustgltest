#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    xyz: [f32; 3]
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            xyz: [x, y, z]
        }
    }

    pub fn x(&self) -> f32 {
        self.xyz[0]
    }
    pub fn y(&self) -> f32 {
        self.xyz[1]
    }
    pub fn z(&self) -> f32 {
        self.xyz[2]
    }

    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        self.xyz[0] = x;
        self.xyz[1] = y;
        self.xyz[2] = z;
    }
    pub fn set_x(&mut self, x: f32) {
        self.xyz[0] = x;
    }
    pub fn set_y(&mut self, y: f32) {
        self.xyz[1] = y;
    }
    pub fn set_z(&mut self, z: f32) {
        self.xyz[2] = z;
    }
}