#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[allow(dead_code)]
impl Colour {

    pub fn rgb(r: u8, g: u8, b: u8) -> Colour {
        Colour { r: r, g: g, b: b }
    }

    pub fn hsv(h: u8, s: u8, v: u8) -> Colour {
        let region: u8;
        let remainder: u8;
        let p: u8;
        let q: u8;
        let t: u8;

        if s == 0 {
            return Colour { r: v, g: v, b: v };
        }

        region = h / 43;
        remainder = (h - (region * 43)) * 6;

        p = (v * (255 - s)) >> 8;
        q = (v * (255 - ((s * remainder) >> 8))) >> 8;
        t = (v * (255 - ((s * (255 - remainder)) >> 8))) >> 8;

        return match region {
            0 => Colour { r: v, g: t, b: p },
            1 => Colour { r: q, g: v, b: p },
            2 => Colour { r: p, g: v, b: t },
            3 => Colour { r: p, g: q, b: v },
            4 => Colour { r: t, g: p, b: v },
            _ => Colour { r: v, g: p, b: q },
        };
    }

    pub fn black() -> Colour {
        Self::rgb(0, 0, 0)
    }

    pub fn white() -> Colour {
        Self::rgb(255, 255, 255)
    }

    pub fn red() -> Colour {
        Self::rgb(255, 0, 0)
    }

    pub fn green() -> Colour {
        Self::rgb(0, 255, 0)
    }

    pub fn blue() -> Colour {
        Self::rgb(0, 0, 255)
    }

    pub fn yellow() -> Colour {
        Self::rgb(255, 255, 0)
    }

    pub fn purple() -> Colour {
        Self::rgb(255, 0, 255)
    }

    pub fn light_blue() -> Colour {
        Self::rgb(0, 255, 255)
    }

}
