const MAX_PATTERNS: usize = 5;

#[derive(Copy, Clone)]
pub struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

impl From<&&str> for RGB {
    fn from(hex: &&str) -> Self {
        let hex = hex.trim_start_matches('#');
        let red = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let green = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let blue = u8::from_str_radix(&hex[4..6], 16).unwrap();
        RGB { red, green, blue }
    }
}

impl RGB {
    pub fn to_rgb8(self, brightness: Option<u8>) -> smart_leds::RGB8 {
        let _percent = brightness.unwrap_or(100);
        return smart_leds::RGB8 {
            r: self.red,
            g: self.green,
            b: self.blue,
        };
    }
}

pub struct LedPattern {
    pub colors: [RGB; MAX_PATTERNS],
}

impl LedPattern {
    pub fn default() -> LedPattern {
        return LedPattern {
            colors: [RGB {
                red: 0,
                green: 0,
                blue: 0,
            }; MAX_PATTERNS],
        };
    }

    pub fn from_list(list: &[&str]) -> LedPattern {
        let mut colors = [RGB {
            red: 0,
            green: 0,
            blue: 0,
        }; MAX_PATTERNS];
        for (i, color) in list.iter().enumerate() {
            colors[i] = RGB::from(color);
        }
        LedPattern { colors }
    }
}
