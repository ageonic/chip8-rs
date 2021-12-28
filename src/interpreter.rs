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

    pub fn read_instruction(&self) -> u16 {
        ((self.memory[self.pc] as u16) << 8) | self.memory[self.pc + 1] as u16
    }
}
