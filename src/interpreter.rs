use crate::instruction::Instruction;

#[derive(Debug)]
pub struct CPU {
    pub memory: [u8; 4096],
    pub v: [u8; 16],
    pub stack: [usize; 16],
    pub pc: usize,
    pub i: usize,
    sound_timer: u8,
    delay_timer: u8,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: [0; 4096],
            v: [0; 16],
            stack: [0; 16],
            pc: 0x200,
            i: 0,
            sound_timer: 0,
            delay_timer: 0,
        }
    }

    pub fn load(&mut self, bytes: &[u8]) {
        for (i, &byte) in bytes.iter().enumerate() {
            let address = 0x200 + i;

            if address < 4096 {
                self.memory[address] = byte;
            } else {
                break;
            }
        }
    }

    pub fn execute(&mut self) {
        let instruction = Instruction::new(self.read_instruction());

        match instruction.as_nibbles() {
            (0x0, 0x0, 0xE, 0x0) => (),
            (0x0, 0x0, 0xE, 0xE) => (),
            (0x0, _, _, _) => (),
            (0x1, _, _, _) => (),
            (0x2, _, _, _) => (),
            (0x3, _, _, _) => (),
            (0x4, _, _, _) => (),
            (0x5, _, _, 0x0) => (),
            (0x6, _, _, _) => (),
            (0x7, _, _, _) => (),
            (0x8, _, _, 0x0) => (),
            (0x8, _, _, 0x1) => (),
            (0x8, _, _, 0x2) => (),
            (0x8, _, _, 0x3) => (),
            (0x8, _, _, 0x4) => (),
            (0x8, _, _, 0x5) => (),
            (0x8, _, _, 0x6) => (),
            (0x8, _, _, 0x7) => (),
            (0x8, _, _, 0xE) => (),
            (0x9, _, _, 0x0) => (),
            (0xA, _, _, _) => (),
            (0xB, _, _, _) => (),
            (0xC, _, _, _) => (),
            (0xD, _, _, _) => (),
            (0xE, _, 0x9, 0xE) => (),
            (0xE, _, 0xA, 0x1) => (),
            (0xF, _, 0x0, 0x7) => (),
            (0xF, _, 0x0, 0xA) => (),
            (0xF, _, 0x1, 0x5) => (),
            (0xF, _, 0x1, 0x8) => (),
            (0xF, _, 0x1, 0xE) => (),
            (0xF, _, 0x2, 0x9) => (),
            (0xF, _, 0x3, 0x3) => (),
            (0xF, _, 0x5, 0x5) => (),
            (0xF, _, 0x6, 0x5) => (),
            (_, _, _, _) => (),
        }
    }

    pub fn read_instruction(&self) -> u16 {
        ((self.memory[self.pc] as u16) << 8) | self.memory[self.pc + 1] as u16
    }
}
