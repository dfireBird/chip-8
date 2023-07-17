mod timer;

use timer::Timer;

use super::HEIGHT;
use super::MAX_MEMORY;
use super::WIDTH;

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

impl Default for CPU {
    fn default() -> Self {
        CPU {
            registers: [0_u8; 16],
            memory: [0_u8; MAX_MEMORY],
            pc: 0x200,
            index: 0,
            stack: Vec::new(),
            framebuffer: [false; WIDTH * HEIGHT],
            sound: Timer::default(),
            delay: Timer::default(),
        }
    }
}
