use std::{fs, time::Duration};

use anyhow::{self, Context};
use minifb::{Key, Scale, Window, WindowOptions};

mod cpu;

use cpu::CPU;

pub const MAX_MEMORY: usize = 4096;
pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

const WINDOW_TITLE: &str = "Chip-Octo";

const CYCLES_PER_FRAME: u32 = 20;

const KEY_MAP: [Key; 16] = [
    Key::X,
    Key::Key1,
    Key::Key2,
    Key::Key3,
    Key::Q,
    Key::W,
    Key::E,
    Key::A,
    Key::S,
    Key::D,
    Key::Z,
    Key::C,
    Key::Key4,
    Key::R,
    Key::F,
    Key::V,
];

pub fn run(rom_path: &str) -> anyhow::Result<()> {
    let program = load_from_file(rom_path).context("Reading ROM from file")?;

    let mut cpu = CPU::init(program);

    let window_opts = WindowOptions {
        scale: Scale::X8,
        ..Default::default()
    };
    let mut window =
        Window::new(WINDOW_TITLE, WIDTH, HEIGHT, window_opts).context("Creating the Window")?;
    window.limit_update_rate(Some(Duration::from_micros(16660)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // input_keys denote the keys that are to be used by Chip-8
        let mut input_keys = Vec::new();

        for key in window.get_keys() {
            if let Some(index) = KEY_MAP.iter().position(|k| *k == key) {
                input_keys.push(index as u8);
            } else {
                // Else condition for keys not related to Chip-8, to be used if needed
            }
        }

        for _ in 0..CYCLES_PER_FRAME {
            cpu.step(&mut input_keys);
        }

        cpu.step_timers();

        window
            .update_with_buffer(&cpu.get_framebuffer(), WIDTH, HEIGHT)
            .context("Updating the window")?;
    }

    Ok(())
}

fn load_from_file(path: &str) -> anyhow::Result<Vec<u8>> {
    Ok(fs::read(path)?)
}
