// Represents a deconstructed opcode
#[derive(Debug)]
pub struct Opcode {
    pub category: usize,
    pub x: usize,
    pub y: usize,
    pub n: usize,
    pub nn: usize,
    pub nnn: usize,
    raw: u16,
}

impl From<u16> for Opcode {
    fn from(raw: u16) -> Self {
        Opcode {
            category: ((raw & 0xF000) >> 12) as usize,
            x: ((raw & 0x0F00) >> 8) as usize,
            y: ((raw & 0x00F0) >> 4) as usize,
            n: (raw & 0x000F) as usize,
            nn: (raw & 0x00FF) as usize,
            nnn: (raw & 0x0FFF) as usize,
            raw,
        }
    }
}

impl Opcode {
    pub fn as_nibbles(&self) -> (usize, usize, usize, usize) {
        (self.category, self.x, self.y, self.n)
    }
}
