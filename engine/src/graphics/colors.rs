use raylib::prelude::Color as RaylibColor;


pub enum Color {
    WHITE,
    BLACK,
    RED,
    GREEN,
    BLUE,
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, u8)
}

impl Color {
    
    pub fn to_rgb(self) -> ColorRgba {
        
        match self {
            Color::WHITE => ColorRgba::WHITE,
            Color::BLACK => ColorRgba::BLACK,
            Color::RED => ColorRgba::RED,
            Color::GREEN => ColorRgba::GREEN,
            Color::BLUE => ColorRgba::BLUE,
            Color::Rgb(r, g, b) => ColorRgba {
                r, g, b, a: 255
            },
            Color::Rgba(r, g, b, a) => ColorRgba {
                r, g, b, a
            }
        } 

    } 

}


pub struct ColorRgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl ColorRgba {
   
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r, g, b, a 
        }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r, g, b, a: 255 
        }
    }

    pub const BLACK: Self = Self {
        r: 0, g: 0, b: 0, a: 255
    };

    pub const WHITE: Self = Self {
        r: 255, g: 255, b: 255, a: 255
    };

    pub const RED: Self = Self {
        r: 255, g: 0, b: 0, a: 255
    };

    pub const GREEN: Self = Self {
        r: 0, g: 255, b: 0, a: 255
    };

    pub const BLUE: Self = Self {
        r: 0, g: 0, b: 255, a: 255
    };

    pub fn to_raylib_color(self) -> RaylibColor {
        RaylibColor {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a
        }
    }

}


#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn color_enum_to_color_rgb() {
   
        // Create an enum, then convert it into a struct 
        let col: Color = Color::WHITE;
        let rgb: ColorRgba = col.to_rgb();
        assert_eq!(rgb.r, 255);
        assert_eq!(rgb.g, 255);
        assert_eq!(rgb.b, 255);
        assert_eq!(rgb.a, 255);

    }

}


