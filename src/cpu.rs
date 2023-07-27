mod decode;
mod opcode;
mod timer;

use std::ops;

use decode::decode_op;
use opcode::OpCode;
use timer::Timer;

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

const OFF_PIXEL_COLOR: u32 = 0xAAAAAA;
const ON_PIXEL_COLOR: u32 = 0x000000;

const FLAG_REGISTER: usize = 0x0F;

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
    pub fn step(&mut self) {
        let first_byte = self.memory[self.pc as usize];
        let second_byte = self.memory[(self.pc + 1) as usize];
        self.pc += 2;

        let opcode = decode_op(first_byte, second_byte);

        self.execute_instruction(opcode);
    }

    pub fn step_timers(&mut self) {
        self.sound.sub();
        self.delay.sub();
    }

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

    pub fn get_framebuffer(&self) -> Vec<u32> {
        let mut op_framebuffer = vec![0; WIDTH * HEIGHT];

        for (i, pixel_data) in self.framebuffer.iter().enumerate() {
            op_framebuffer[i] = if *pixel_data {
                ON_PIXEL_COLOR
            } else {
                OFF_PIXEL_COLOR
            }
        }

        op_framebuffer
    }

    fn execute_instruction(&mut self, opcode: OpCode) {
        use OpCode::*;
        match opcode {
            Cls => self.framebuffer = [false; WIDTH * HEIGHT],
            Ret => self.pc = self.stack.pop().expect("Stack is empty"),
            Jmp(addr) => self.pc = addr,
            Call(addr) => {
                self.stack.push(self.pc);
                self.pc = addr;
            }
            SkipEq(x, val) => self.skip(self.registers[x as usize], val, PartialEq::eq),
            SkipNotEq(x, val) => self.skip(self.registers[x as usize], val, PartialEq::ne),
            SkipRegEq(x, y) => self.skip(
                self.registers[x as usize],
                self.registers[y as usize],
                PartialEq::eq,
            ),
            SkipRegNotEq(x, y) => self.skip(
                self.registers[x as usize],
                self.registers[y as usize],
                PartialEq::ne,
            ),
            Set(x_reg, val) => self.registers[x_reg as usize] = val,
            AddImd(x_reg, val) => self.registers[x_reg as usize] += val,
            SetReg(x, y) => self.registers[x as usize] = self.registers[y as usize],
            Or(x, y) => self.logical(x, y, ops::BitOr::bitor),
            And(x, y) => self.logical(x, y, ops::BitAnd::bitand),
            Xor(x, y) => self.logical(x, y, ops::BitXor::bitxor),
            Add(x, y) => {
                let (add_val, is_overflow) =
                    self.registers[x as usize].overflowing_add(self.registers[y as usize]);
                self.registers[x as usize] = add_val;

                self.registers[FLAG_REGISTER] = is_overflow.into();
            }
            Sub(x, y) => {
                let (sub_val, is_overflow) =
                    self.registers[x as usize].overflowing_sub(self.registers[y as usize]);
                self.registers[x as usize] = sub_val;

                self.registers[FLAG_REGISTER] = (!is_overflow).into();
            }
            SubInv(x, y) => {
                let (sub_val, is_overflow) =
                    self.registers[y as usize].overflowing_sub(self.registers[x as usize]);
                self.registers[x as usize] = sub_val;

                self.registers[FLAG_REGISTER] = (!is_overflow).into();
            }
            ShiftL(x, y) => {
                let val = self.registers[y as usize];
                self.registers[FLAG_REGISTER] = val & 0x01;
                self.registers[x as usize] = val << 1;
            }
            ShiftR(x, y) => {
                let val = self.registers[y as usize];
                self.registers[FLAG_REGISTER] = val & 0x80;
                self.registers[x as usize] = val >> 1;
            }
            SetIndex(addr) => self.index = addr,
            JmpOff(addr) => self.pc = addr + self.registers[0] as u16,
            Rand(x, val) => self.random(x, val),
            Disp(x_reg, y_reg, n_bytes) => self.display(x_reg, y_reg, n_bytes),
            SetFromDelay(x) => self.registers[x as usize] = self.delay.get(),
            SetDelay(x) => self.delay.set(self.registers[x as usize]),
            SetSound(x) => self.sound.set(self.registers[x as usize]),
            AddIndex(x) => self.index += self.registers[x as usize] as u16,
            Font(x) => {
                self.index = FONT_START_ADDRESS as u16 + (self.registers[x as usize] * 5) as u16
            }
            DecConv(x) => self.dec_conv(x),
            Store(x) => {
                let index = self.index as usize;
                let x = x as usize;

                for i in 0..=x {
                    self.memory[index + i] = self.registers[i];
                }
            }
            Load(x) => {
                let index = self.index as usize;
                let x = x as usize;

                for i in 0..=x {
                    self.registers[i] = self.memory[index + i];
                }
            }
            _ => panic!("Instruction not implemented"), // Panic for now later implement proper error handling
        }
    }

    fn skip(&mut self, rhs: u8, lhs: u8, op: impl Fn(&u8, &u8) -> bool) {
        if op(&rhs, &lhs) {
            self.pc += 2;
        }
    }

    fn logical(&mut self, x: u8, y: u8, op: impl Fn(u8, u8) -> u8) {
        let lhs = self.registers[x as usize];
        let rhs = self.registers[y as usize];
        self.registers[x as usize] = op(lhs, rhs);
    }

    fn random(&mut self, x: u8, val: u8) {
        let rand_value: u8 = rand::random();
        self.registers[x as usize] = rand_value & val;
    }

    fn dec_conv(&mut self, x: u8) {
        let mut val = self.registers[x as usize];
        let mut adder = 0;
        while val > 0 {
            let digit = val % 10;

            self.memory[(self.index + adder) as usize] = digit;

            val /= 10;
            adder += 1;
        }
    }

    fn display(&mut self, reg_x: u8, reg_y: u8, n_bytes: u8) {
        let n_bytes = n_bytes as u16;
        let (reg_x, reg_y) = (reg_x as usize, reg_y as usize);

        let coord_x = (self.registers[reg_x] % WIDTH as u8) as usize;
        let coord_y = (self.registers[reg_y] % HEIGHT as u8) as usize;

        for i in 0..n_bytes {
            let sprite_data_byte = self.memory[(self.index + i) as usize];

            let mut cur_coord_x = coord_x;
            let cur_coord_y = coord_y + i as usize;

            for bit in convert_into_bit_list(sprite_data_byte) {
                let index = cur_coord_x + WIDTH * cur_coord_y;

                let old_bit = self.framebuffer[index];
                let new_bit = old_bit ^ bit;

                self.framebuffer[index] = new_bit;
                if !new_bit {
                    self.registers[FLAG_REGISTER] = 1;
                }

                cur_coord_x += 1;
            }
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

fn convert_into_bit_list(byte: u8) -> [bool; 8] {
    let mut bit_list = [false; 8];
    let mut mask = 0x01;

    for bit in bit_list.iter_mut().rev() {
        *bit = byte & mask != 0;
        mask = mask << 0x01;
    }

    bit_list
}
