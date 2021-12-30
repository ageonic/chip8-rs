// Represents a deconstructed opcode
#[derive(Debug)]
pub struct Instruction {
    pub code: u16,
    category: usize,
    pub x: usize,
    pub y: usize,
    pub n: usize,
    pub nn: usize,
    pub nnn: usize,
}

impl Instruction {
    pub fn new(opcode: u16) -> Instruction {
        Instruction {
            code: opcode,
            category: ((opcode & 0xF000) >> 12) as usize,
            x: ((opcode & 0x0F00) >> 8) as usize,
            y: ((opcode & 0x00F0) >> 4) as usize,
            n: (opcode & 0x000F) as usize,
            nn: (opcode & 0x00FF) as usize,
            nnn: (opcode & 0x0FFF) as usize,
        }
    }

    pub fn as_nibbles(&self) -> (usize, usize, usize, usize) {
        (self.category, self.x, self.y, self.n)
    }
}
