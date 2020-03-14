#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[allow(dead_code)]
impl Colour {

    pub fn new(r: u8, g: u8, b: u8) -> Colour {
        Colour { r: r, g: g, b: b }
    }

    pub fn black() -> Colour {
        Self::new(0, 0, 0)
    }

    pub fn white() -> Colour {
        Self::new(255, 255, 255)
    }

    pub fn red() -> Colour {
        Self::new(255, 0, 0)
    }

    pub fn green() -> Colour {
        Self::new(0, 255, 0)
    }

    pub fn blue() -> Colour {
        Self::new(0, 0, 255)
    }

    pub fn yellow() -> Colour {
        Self::new(255, 255, 0)
    }

    pub fn purple() -> Colour {
        Self::new(255, 0, 255)
    }

    pub fn light_blue() -> Colour {
        Self::new(0, 255, 255)
    }

}
