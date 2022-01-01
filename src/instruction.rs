// Represents a deconstructed opcode
#[derive(Debug)]
pub struct Opcode {
    category: usize,
    x: usize,
    y: usize,
    n: usize,
    nn: usize,
    nnn: usize,
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
    pub fn as_u16(&self) -> u16 {
        self.raw
    }

    pub fn as_nibbles(&self) -> (usize, usize, usize, usize) {
        (self.category, self.x, self.y, self.n)
    }

    pub fn as_destructured_params(&self) -> (usize, usize, usize, usize, usize) {
        (self.x, self.y, self.n, self.nn, self.nnn)
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::Opcode;

    #[test]
    fn from_interprets_00e0() {
        let op = Opcode::from(0x00E0);
        assert_eq!(0x0, op.category);
        assert_eq!(0x0, op.x);
        assert_eq!(0xE, op.y);
        assert_eq!(0x0, op.n);
        assert_eq!(0xE0, op.nn);
        assert_eq!(0x0E0, op.nnn);
    }

    #[test]
    fn from_interprets_a2ec() {
        let op = Opcode::from(0xA2EC);
        assert_eq!(0xA, op.category);
        assert_eq!(0x2, op.x);
        assert_eq!(0xE, op.y);
        assert_eq!(0xC, op.n);
        assert_eq!(0xEC, op.nn);
        assert_eq!(0x2EC, op.nnn);
    }

    #[test]
    fn from_interprets_87a1() {
        let op = Opcode::from(0x87A1);
        assert_eq!(0x8, op.category);
        assert_eq!(0x7, op.x);
        assert_eq!(0xA, op.y);
        assert_eq!(0x1, op.n);
        assert_eq!(0xA1, op.nn);
        assert_eq!(0x7A1, op.nnn);
    }

    #[test]
    fn as_u16_returns_raw_opcode() {
        let op = Opcode::from(0xA32E);
        assert_eq!(0xA32E, op.as_u16());
    }

    #[test]
    fn as_nibbles_returns_expected_tuple() {
        let op = Opcode::from(0xA32E);
        assert_eq!((0xA, 0x3, 0x2, 0xE), op.as_nibbles());
    }

    #[test]
    fn as_destructured_params_returns_expected_tuple() {
        let op = Opcode::from(0xA32E);
        assert_eq!((0x3, 0x2, 0xE, 0x2E, 0x32E), op.as_destructured_params());
    }
}
