mod timer;

use timer::Timer;

use crate::decode::decode_op;

use super::HEIGHT;
use super::MAX_MEMORY;
use super::WIDTH;

const PROGRAM_START_ADDRESS: usize = 0x200;
const FONT_START_ADDRESS: usize = 0x050;

const FONT_DATA: [[u8; 5]; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
    [0x20, 0x60, 0x20, 0x20, 0x70], // 1
    [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
    [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
    [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
    [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
    [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
    [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
    [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
    [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
    [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
    [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
    [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
    [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
    [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
    [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
];

pub struct CPU {
    registers: [u8; 16],
    memory: [u8; MAX_MEMORY],
    pc: u16,
    index: u16,
    stack: Vec<u16>,
    framebuffer: [bool; WIDTH * HEIGHT],
    sound: Timer,
    delay: Timer,
}

impl CPU {
    pub fn init(program: Vec<u8>) -> Self {
        let mut memory = [0_u8; MAX_MEMORY];
        let mut cur_addr = FONT_START_ADDRESS;

        for font in FONT_DATA {
            for byte in font {
                memory[cur_addr] = byte;
                cur_addr += 1;
            }
        }

        cur_addr = PROGRAM_START_ADDRESS;
        for byte in program {
            memory[cur_addr] = byte;
            cur_addr += 1;
        }

        Self {
            memory,
            ..Default::default()
        }
    }
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            registers: [0_u8; 16],
            memory: [0_u8; MAX_MEMORY],
            pc: PROGRAM_START_ADDRESS as u16,
            index: 0,
            stack: Vec::new(),
            framebuffer: [false; WIDTH * HEIGHT],
            sound: Timer::default(),
            delay: Timer::default(),
        }
    }
}
