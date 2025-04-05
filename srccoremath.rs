use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn distance(&self, other: &Self) -> f32 {
        (*self - *other).length()
    }

    pub fn normalize(&mut self) {
        let len = self.length();
        if len > 0.0 {
            self.x /= len;
            self.y /= len;
        }
    }

    pub fn normalized(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Vec2 { x: self.x / len, y: self.y / len }
        } else {
            *self
        }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec2 { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Vec2 { x: self.x * scalar, y: self.y * scalar }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        if scalar != 0.0 {
            Vec2 { x: self.x / scalar, y: self.y / scalar }
        } else {
            panic!("Cannot divide by zero!");
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn distance(&self, other: &Self) -> f32 {
        (*self - *other).length()
    }

    pub fn normalize(&mut self) {
        let len = self.length();
        if len > 0.0 {
            self.x /= len;
            self.y /= len;
            self.z /= len;
        }
    }

    pub fn normalized(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Vec3 { x: self.x / len, y: self.y / len, z: self.z / len }
        } else {
            *self
        }
    }

    pub fn angle(&self, other: &Self) -> f32 {
        let dot = dot_product(self, other);
        let magnitudes = self.length() * other.length();
        if magnitudes > 0.0 {
            (dot / magnitudes).acos()
        } else {
            0.0
        }
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        let dot = dot_product(self, normal);
        *self - *normal * 2.0 * dot
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Vec3 { x: self.x * scalar, y: self.y * scalar, z: self.z * scalar }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        if scalar != 0.0 {
            Vec3 { x: self.x / scalar, y: self.y / scalar, z: self.z / scalar }
        } else {
            panic!("Cannot divide by zero!");
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat4 {
    pub elements: [[f32; 4]; 4],
}

impl Mat4 {
    pub fn identity() -> Self {
        Mat4 {
            elements: [
               ,
               ,
               ,
               ,
            ],
        }
    }

    pub fn translate(translation: &Vec3) -> Self {
        Mat4 {
            elements: [
                [1.0, 0.0, 0.0, translation.x],
                [0.0, 1.0, 0.0, translation.y],
                [0.0, 0.0, 1.0, translation.z],
               ,
            ],
        }
    }

    pub fn rotate_x(angle_rad: f32) -> Self {
        let c = angle_rad.cos();
        let s = angle_rad.sin();
        Mat4 {
            elements: [
               ,
                [0.0, c, -s, 0.0],
                [0.0, s, c, 0.0],
               ,
            ],
        }
    }

    pub fn rotate_y(angle_rad: f32) -> Self {
        let c = angle_rad.cos();
        let s = angle_rad.sin();
        Mat4 {
            elements: [
                [c, 0.0, s, 0.0],
               ,
                [-s, 0.0, c, 0.0],
               ,
            ],
        }
    }

    pub fn rotate_z(angle_rad: f32) -> Self {
        let c = angle_rad.cos();
        let s = angle_rad.sin();
        Mat4 {
            elements: [
                [c, -s, 0.0, 0.0],
                [s, c, 0.0, 0.0],
               ,
               ,
            ],
        }
    }

    pub fn scale(scale: &Vec3) -> Self {
        Mat4 {
            elements: [
                [scale.x, 0.0, 0.0, 0.0],
                [0.0, scale.y, 0.0, 0.0],
                [0.0, 0.0, scale.z, 0.0],
               ,
            ],
        }
    }
}

impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = Mat4::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.elements[i][j] = self.elements[i][0] * other.elements[0][j] +
                                         self.elements[i][1] * other.elements[1][j] +
                                         self.elements[i][2] * other.elements[2][j] +
                                         self.elements[i][3] * other.elements[3][j];
            }
        }
        result
    }
}

impl Mul<Vec3> for Mat4 {
    type Output = Vec3;

    fn mul(self, vector: Vec3) -> Vec3 {
        Vec3 {
            x: self.elements[0][0] * vector.x + self.elements[0][1] * vector.y + self.elements[0][2] * vector.z + self.elements[0][3],
            y: self.elements[1][0] * vector.x + self.elements[1][1] * vector.y + self.elements[1][2] * vector.z + self.elements[1][3],
            z: self.elements[2][0] * vector.x + self.elements[2][1] * vector.y + self.elements[2][2] * vector.z + self.elements[2][3],
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32, // Skalar kısım genellikle 'w' ile gösterilir
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Quaternion { x, y, z, w }
    }

    pub fn identity() -> Self {
        Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn normalize(&mut self) {
        let len = self.length();
        if len > 0.0 {
            self.x /= len;
            self.y /= len;
            self.z /= len;
            self.w /= len;
        }
    }

    pub fn normalized(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Quaternion { x: self.x / len, y: self.y / len, z: self.z / len, w: self.w / len }
        } else {
            *self
        }
    }

    pub fn conjugate(&self) -> Self {
        Quaternion { x: -self.x, y: -self.y, z: -self.z, w: self.w }
    }

    pub fn from_axis_angle(axis: &Vec3, angle_rad: f32) -> Self {
        let half_angle = angle_rad / 2.0;
        let sin_half_angle = half_angle.sin();
        let normalized_axis = axis.normalized();
        Quaternion {
            x: normalized_axis.x * sin_half_angle,
            y: normalized_axis.y * sin_half_angle,
            z: normalized_axis.z * sin_half_angle,
            w: half_angle.cos(),
        }
    }

    pub fn to_rotation_matrix(&self) -> Mat4 {
        let xx = self.x * self.x;
        let xy = self.x * self.y;
        let xz = self.x * self.z;
        let xw = self.x * self.w;
        let yy = self.y * self.y;
        let yz = self.y * self.z;
        let yw = self.y * self.w;
        let zz = self.z * self.z;
        let zw = self.z * self.w;

        Mat4 {
            elements: [
                [1.0 - 2.0 * (yy + zz), 2.0 * (xy - zw), 2.0 * (xz + yw), 0.0],
                [2.0 * (xy + zw), 1.0 - 2.0 * (xx + zz), 2.0 * (yz - xw), 0.0],
                [2.0 * (xz - yw), 2.0 * (yz + xw), 1.0 - 2.0 * (xx + yy), 0.0],
               ,
            ],
        }
    }

    pub fn rotate_vector(&self, vector: &Vec3) -> Vec3 {
        let conjugate = self.conjugate();
        let q_vector = Quaternion::new(vector.x, vector.y, vector.z, 0.0);
        let rotated_q = *self * q_vector * conjugate;
        Vec3::new(rotated_q.x, rotated_q.y, rotated_q.z)
    }
}

impl Mul for Quaternion {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Quaternion {
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
        }
    }
}

// Nokta Çarpımı (Dot Product) - Vec3 için
pub fn dot_product(a: &Vec3, b: &Vec3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

// Çapraz Çarpım (Cross Product) - Vec3 için
pub fn cross_product(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

// Nokta Çarpımı (Dot Product) - Vec2 için (isteğe bağlı olarak eklenebilir)
pub fn dot_product_vec2(a: &Vec2, b: &Vec2) -> f32 {
    a.x * b.x + a.y * b.y
}