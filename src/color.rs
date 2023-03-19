#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Color {
    hue: f32,
    saturation: f32,
    value: f32
}

impl Color {
    pub fn update_hue(&self, hue: f32) -> Color {
        Color { hue, saturation: self.saturation, value: self.value }
    }

    pub fn update_saturation(&self, saturation: f32) -> Color {
        Color { saturation, hue: self.hue, value: self.value }
    }

    pub fn update_value(&self, value: f32) -> Color {
        Color { value, saturation: self.saturation, hue: self.hue }
    }

    pub fn from_hsv(hue: f32, saturation: f32, value: f32) -> Color {
        Color { hue, saturation, value }
    }

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Color {
        let c_max = r.max(g.max(b));
        let c_min = r.min(g.min(b));

        let value = c_max;

        if c_max == c_min {
            return Color { hue: 0.0, saturation: 0.0, value };
        }

        let saturation = (c_max - c_min) / c_max;

        let rc = (c_max - r) / (c_max - c_min);
        let gc = (c_max - g) / (c_max - c_min);
        let bc = (c_max - b) / (c_max - c_min);

        let hue = if r == c_max {
            bc - gc
        } else if g == c_max {
            2.0 + rc - bc
        } else {
            4.0 + gc - rc
        } / 6.0;

        Color { hue, saturation, value }
    }

    pub fn to_rgb_u8(&self) -> (u8, u8, u8) {
        let r;
        let g;
        let b;

        let i = (self.hue * 6.0).floor() as u8;
        let f = self.hue * 6.0 - i as f32;
        let p = self.value * (1.0 - self.saturation);
        let q = self.value * (1.0 - f * self.saturation);
        let t = self.value * (1.0 - (1.0 - f) * self.saturation);

        match i {
            0 | 6 => {
                r = self.value;
                g = t;
                b = p;
            },
            1 => {
                r = q;
                g = self.value;
                b = p;
            },
            2 => {
                r = p;
                g = self.value;
                b = t;
            },
            3 => {
                r = p;
                g = q;
                b = self.value;
            },
            4 => {
                r = t;
                g = p;
                b = self.value;
            },
            5 => {
                r = self.value;
                g = p;
                b = q; 
            },
            _ => { panic!("self.hue > 1.0") }
        };

        ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
    }

    pub fn to_hex(&self) -> String {
        let (r, g, b) = self.to_rgb_u8();

        let char_lut = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
        let mut result = String::new();

        // hashtag prefix
        result.push('#');

        // red component
        result.push(char_lut[(r / 16) as usize]);
        result.push(char_lut[(r % 16) as usize]);

        // green component
        result.push(char_lut[(g / 16) as usize]);
        result.push(char_lut[(g % 16) as usize]);

        // blue component
        result.push(char_lut[(b / 16) as usize]);
        result.push(char_lut[(b % 16) as usize]);

        result
    }
}
