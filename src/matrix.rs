use std::ops;
use crate::vector::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Mat4 {
    pub a: [[f32; 4]; 4]
}

impl Mat4 {
    pub fn identity() -> Mat4 {
        Mat4 {
            a: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        }
    }
    pub fn new(values: [[f32; 4]; 4]) -> Mat4 {
        Mat4 {
            a: values
        }
    }

    pub fn scale_uniform(scale: f32) -> Mat4 {
        Mat4 {
            a: [
                [ scale, 0.0, 0.0, 0.0 ],
                [ 0.0, scale, 0.0, 0.0 ],
                [ 0.0, 0.0, scale, 0.0 ],
                [ 0.0, 0.0, 0.0, 1.0 ]
            ]
        }
    }
    pub fn scale_xy(x: f32, y: f32) -> Mat4 {
        Mat4 {
            a: [
                [ x, 0.0, 0.0, 0.0 ],
                [ 0.0, y, 0.0, 0.0 ],
                [ 0.0, 0.0, 1.0, 0.0 ],
                [ 0.0, 0.0, 0.0, 1.0 ]
            ]
        }
    }
    pub fn translate(x: f32, y :f32, z: f32) -> Mat4 {
        Mat4 {
            a: [
                [ 1.0, 0.0, 0.0, x ],
                [ 0.0, 1.0, 0.0, y ],
                [ 0.0, 0.0, 1.0, z ],
                [ 0.0, 0.0, 0.0, 1.0 ]
            ]
        }
    }

    /// Row and Column for Matrix accessing are 1-based index
    pub fn get(&self, row: usize, col: usize) -> Option<f32> {
        if row > 4 || col > 4 {
            return None;
        }

        Some(self.a[row-1][col-1])
    }
}

impl ops::Mul<Mat4> for Mat4 {
    type Output = Mat4;
    fn mul(self, rhs: Mat4) -> Mat4 {
        Mat4 {
            a: [
                [
                    self.a[0][0] * rhs.a[0][0] + self.a[0][1] * rhs.a[1][0] + self.a[0][2] * rhs.a[2][0] + self.a[0][3] * rhs.a[3][0],
                    self.a[0][0] * rhs.a[0][1] + self.a[0][1] * rhs.a[1][1] + self.a[0][2] * rhs.a[2][1] + self.a[0][3] * rhs.a[3][1],
                    self.a[0][0] * rhs.a[0][2] + self.a[0][1] * rhs.a[1][2] + self.a[0][2] * rhs.a[2][2] + self.a[0][3] * rhs.a[3][2],
                    self.a[0][0] * rhs.a[0][3] + self.a[0][1] * rhs.a[1][3] + self.a[0][2] * rhs.a[2][3] + self.a[0][3] * rhs.a[3][3]
                ],
                [
                    self.a[1][0] * rhs.a[0][0] + self.a[1][1] * rhs.a[1][0] + self.a[1][2] * rhs.a[2][0] + self.a[1][3] * rhs.a[3][0],
                    self.a[1][0] * rhs.a[0][1] + self.a[1][1] * rhs.a[1][1] + self.a[1][2] * rhs.a[2][1] + self.a[1][3] * rhs.a[3][1],
                    self.a[1][0] * rhs.a[0][2] + self.a[1][1] * rhs.a[1][2] + self.a[1][2] * rhs.a[2][2] + self.a[1][3] * rhs.a[3][2],
                    self.a[1][0] * rhs.a[0][3] + self.a[1][1] * rhs.a[1][3] + self.a[1][2] * rhs.a[2][3] + self.a[1][3] * rhs.a[3][3]
                ],
                [
                    self.a[2][0] * rhs.a[0][0] + self.a[2][1] * rhs.a[1][0] + self.a[2][2] * rhs.a[2][0] + self.a[2][3] * rhs.a[3][0],
                    self.a[2][0] * rhs.a[0][1] + self.a[2][1] * rhs.a[1][1] + self.a[2][2] * rhs.a[2][1] + self.a[2][3] * rhs.a[3][1],
                    self.a[2][0] * rhs.a[0][2] + self.a[2][1] * rhs.a[1][2] + self.a[2][2] * rhs.a[2][2] + self.a[2][3] * rhs.a[3][2],
                    self.a[2][0] * rhs.a[0][3] + self.a[2][1] * rhs.a[1][3] + self.a[2][2] * rhs.a[2][3] + self.a[2][3] * rhs.a[3][3]
                ],
                [
                    self.a[3][0] * rhs.a[0][0] + self.a[3][1] * rhs.a[1][0] + self.a[3][2] * rhs.a[2][0] + self.a[3][3] * rhs.a[3][0],
                    self.a[3][0] * rhs.a[0][1] + self.a[3][1] * rhs.a[1][1] + self.a[3][2] * rhs.a[2][1] + self.a[3][3] * rhs.a[3][1],
                    self.a[3][0] * rhs.a[0][2] + self.a[3][1] * rhs.a[1][2] + self.a[3][2] * rhs.a[2][2] + self.a[3][3] * rhs.a[3][2],
                    self.a[3][0] * rhs.a[0][3] + self.a[3][1] * rhs.a[1][3] + self.a[3][2] * rhs.a[2][3] + self.a[3][3] * rhs.a[3][3]
                ]
            ]
        }
    }
}

impl ops::Mul<Vec3> for Mat4 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        let x: f32 = self.a[0][0] * rhs.x() + self.a[0][1] * rhs.y() + self.a[0][2] * rhs.z() + self.a[0][3];
        let y: f32 = self.a[1][0] * rhs.x() + self.a[1][1] * rhs.y() + self.a[1][2] * rhs.z() + self.a[1][3];
        let z: f32 = self.a[2][0] * rhs.x() + self.a[2][1] * rhs.y() + self.a[2][2] * rhs.z() + self.a[2][3];

        Vec3::new(x, y, z)
    }
}