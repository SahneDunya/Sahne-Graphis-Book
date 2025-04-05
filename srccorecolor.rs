#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color8Bit {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color8Bit {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color8Bit { r, g, b }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color8BitA {
        Color8BitA { r, g, b, a }
    }

    pub fn as_f32(&self) -> (f32, f32, f32) {
        (
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color8BitA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color8BitA {
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color8BitA { r, g, b, a }
    }

    pub fn as_f32(&self) -> (f32, f32, f32, f32) {
        (
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        )
    }

    pub fn as_rgb(&self) -> Color8Bit {
        Color8Bit { r: self.r, g: self.g, b: self.b }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color10Bit {
    pub r: u16, // Aslında 10 bitlik değerler tutacak (0-1023)
    pub g: u16,
    pub b: u16,
}

impl Color10Bit {
    pub fn rgb(r: u16, g: u16, b: u16) -> Self {
        // Burada giriş değerlerinin 0-1023 aralığında olduğunu varsayıyoruz veya kontrol edebiliriz.
        Color10Bit { r, g, b }
    }

    pub fn rgba(r: u16, g: u16, b: u16, a: u16) -> Color10BitA {
        Color10BitA { r, g, b, a }
    }

    pub fn as_f32(&self) -> (f32, f32, f32) {
        (
            self.r as f32 / 1023.0,
            self.g as f32 / 1023.0,
            self.b as f32 / 1023.0,
        )
    }

    // İsteğe bağlı: 8 bit'e dönüştürme
    pub fn as_8bit(&self) -> Color8Bit {
        Color8Bit {
            r: (self.r as f32 / 1023.0 * 255.0) as u8,
            g: (self.g as f32 / 1023.0 * 255.0) as u8,
            b: (self.b as f32 / 1023.0 * 255.0) as u8,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color10BitA {
    pub r: u16, // Aslında 10 bitlik değerler tutacak (0-1023)
    pub g: u16,
    pub b: u16,
    pub a: u16,
}

impl Color10BitA {
    pub fn rgba(r: u16, g: u16, b: u16, a: u16) -> Self {
        Color10BitA { r, g, b, a }
    }

    pub fn as_f32(&self) -> (f32, f32, f32, f32) {
        (
            self.r as f32 / 1023.0,
            self.g as f32 / 1023.0,
            self.b as f32 / 1023.0,
            self.a as f32 / 1023.0,
        )
    }

    pub fn as_rgb(&self) -> Color10Bit {
        Color10Bit { r: self.r, g: self.g, b: self.b }
    }

    pub fn as_8bit(&self) -> Color8BitA {
        Color8BitA {
            r: (self.r as f32 / 1023.0 * 255.0) as u8,
            g: (self.g as f32 / 1023.0 * 255.0) as u8,
            b: (self.b as f32 / 1023.0 * 255.0) as u8,
            a: (self.a as f32 / 1023.0 * 255.0) as u8,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color12Bit {
    pub r: u16, // Aslında 12 bitlik değerler tutacak (0-4095)
    pub g: u16,
    pub b: u16,
}

impl Color12Bit {
    pub fn rgb(r: u16, g: u16, b: u16) -> Self {
        // Burada giriş değerlerinin 0-4095 aralığında olduğunu varsayıyoruz veya kontrol edebiliriz.
        Color12Bit { r, g, b }
    }

    pub fn rgba(r: u16, g: u16, b: u16, a: u16) -> Color12BitA {
        Color12BitA { r, g, b, a }
    }

    pub fn as_f32(&self) -> (f32, f32, f32) {
        (
            self.r as f32 / 4095.0,
            self.g as f32 / 4095.0,
            self.b as f32 / 4095.0,
        )
    }

    // İsteğe bağlı: 8 bit'e dönüştürme
    pub fn as_8bit(&self) -> Color8Bit {
        Color8Bit {
            r: (self.r as f32 / 4095.0 * 255.0) as u8,
            g: (self.g as f32 / 4095.0 * 255.0) as u8,
            b: (self.b as f32 / 4095.0 * 255.0) as u8,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color12BitA {
    pub r: u16, // Aslında 12 bitlik değerler tutacak (0-4095)
    pub g: u16,
    pub b: u16,
    pub a: u16,
}

impl Color12BitA {
    pub fn rgba(r: u16, g: u16, b: u16, a: u16) -> Self {
        Color12BitA { r, g, b, a }
    }

    pub fn as_f32(&self) -> (f32, f32, f32, f32) {
        (
            self.r as f32 / 4095.0,
            self.g as f32 / 4095.0,
            self.b as f32 / 4095.0,
            self.a as f32 / 4095.0,
        )
    }

    pub fn as_rgb(&self) -> Color12Bit {
        Color12Bit { r: self.r, g: self.g, b: self.b }
    }

    pub fn as_8bit(&self) -> Color8BitA {
        Color8BitA {
            r: (self.r as f32 / 4095.0 * 255.0) as u8,
            g: (self.g as f32 / 4095.0 * 255.0) as u8,
            b: (self.b as f32 / 4095.0 * 255.0) as u8,
            a: (self.a as f32 / 4095.0 * 255.0) as u8,
        }
    }
}

// HDR için Float tabanlı renk yapısı
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ColorHDR {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl ColorHDR {
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        ColorHDR { r, g, b }
    }

    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> ColorHDRA {
        ColorHDRA { r, g, b, a }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ColorHDRA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl ColorHDRA {
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        ColorHDRA { r, g, b, a }
    }

    pub fn as_rgb(&self) -> ColorHDR {
        ColorHDR { r: self.r, g: self.g, b: self.b }
    }
}

// HSV Renk Uzayı
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ColorHSV {
    pub h: f32, // 0.0 - 360.0 (derece)
    pub s: f32, // 0.0 - 1.0
    pub v: f32, // 0.0 - 1.0
}

impl ColorHSV {
    pub fn hsv(h: f32, s: f32, v: f32) -> Self {
        ColorHSV { h, s, v }
    }

    // HSV'den RGB'ye dönüşüm (basit bir implementasyon)
    pub fn as_rgb(&self) -> Color {
        let h = self.h / 60.0;
        let s = self.s;
        let v = self.v;
        let i = h.floor() as i32;
        let f = h - i as f32;
        let p = v * (1.0 - s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);

        match i % 6 {
            0 => Color::rgb((v * 255.0) as u8, (t * 255.0) as u8, (p * 255.0) as u8),
            1 => Color::rgb((q * 255.0) as u8, (v * 255.0) as u8, (p * 255.0) as u8),
            2 => Color::rgb((p * 255.0) as u8, (v * 255.0) as u8, (t * 255.0) as u8),
            3 => Color::rgb((p * 255.0) as u8, (q * 255.0) as u8, (v * 255.0) as u8),
            4 => Color::rgb((t * 255.0) as u8, (p * 255.0) as u8, (v * 255.0) as u8),
            5 => Color::rgb((v * 255.0) as u8, (p * 255.0) as u8, (q * 255.0) as u8),
            _ => Color::rgb(0, 0, 0), // Bu durumun olmaması gerekir
        }
    }
}

// HSL Renk Uzayı
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ColorHSL {
    pub h: f32, // 0.0 - 360.0 (derece)
    pub s: f32, // 0.0 - 1.0
    pub l: f32, // 0.0 - 1.0
}

impl ColorHSL {
    pub fn hsl(h: f32, s: f32, l: f32) -> Self {
        ColorHSL { h, s, l }
    }

    // HSL'den RGB'ye dönüşüm (basit bir implementasyon)
    pub fn as_rgb(&self) -> Color {
        let h = self.h / 360.0;
        let s = self.s;
        let l = self.l;

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;
        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;

        match (h * 6.0).floor() as i32 {
            0 => { r = c; g = x; b = 0.0; }
            1 => { r = x; g = c; b = 0.0; }
            2 => { r = 0.0; g = c; b = x; }
            3 => { r = 0.0; g = x; b = c; }
            4 => { r = x; g = 0.0; b = c; }
            5 => { r = c; g = 0.0; b = x; }
            _ => {}
        }

        Color::rgb(((r + m) * 255.0) as u8, ((g + m) * 255.0) as u8, ((b + m) * 255.0) as u8)
    }
}

// Sabit Renkler
pub mod constants {
    use super::*;

    pub const WHITE: Color = Color::rgb(255, 255, 255);
    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const RED: Color = Color::rgb(255, 0, 0);
    pub const GREEN: Color = Color::rgb(0, 255, 0);
    pub const BLUE: Color = Color::rgb(0, 0, 255);
    pub const YELLOW: Color = Color::rgb(255, 255, 0);
    pub const MAGENTA: Color = Color::rgb(255, 0, 255);
    pub const CYAN: Color = Color::rgb(0, 255, 255);
}

// Operatör Overloading (Color8Bit için örnek)
impl std::ops::Add for Color8Bit {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Color8Bit {
            r: self.r.saturating_add(other.r),
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b),
        }
    }
}

impl std::ops::Sub for Color8Bit {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Color8Bit {
            r: self.r.saturating_sub(other.r),
            g: self.g.saturating_sub(other.g),
            b: self.b.saturating_sub(other.b),
        }
    }
}

impl std::ops::Mul<f32> for Color8Bit {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Color8Bit {
            r: (self.r as f32 * scalar).min(255.0).max(0.0) as u8,
            g: (self.g as f32 * scalar).min(255.0).max(0.0) as u8,
            b: (self.b as f32 * scalar).min(255.0).max(0.0) as u8,
        }
    }
}

// Önceki Color tanımını Color8Bit olarak yeniden adlandırdık.
pub type Color = Color8Bit;