use std::{fs, time::Duration};

use anyhow::{self, Context};
use minifb::{Key, Scale, Window, WindowOptions};

mod cpu;

use cpu::CPU;

pub const MAX_MEMORY: usize = 4096;
pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

const WINDOW_TITLE: &str = "Chip-8";

const CYCLES_PER_FRAME: u32 = 20;

pub fn run(rom_path: &str) -> anyhow::Result<()> {
    let program = load_from_file(rom_path).context("Reading ROM from file")?;

    let mut cpu = CPU::init(program);

    let window_opts = WindowOptions {
        scale: Scale::X8,
        ..Default::default()
    };
    let mut window = Window::new(WINDOW_TITLE, WIDTH, HEIGHT, window_opts).unwrap(); // .context("Creating the Window")?;
    window.limit_update_rate(Some(Duration::from_micros(16660)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // TODO: Input Handling
        // No input handling for now as there is no instruction that requires it

        for _ in 0..CYCLES_PER_FRAME {
            cpu.step();
        }

        window
            .update_with_buffer(&cpu.get_framebuffer(), WIDTH, HEIGHT)
            .context("Updating the window")?;
    }

    Ok(())
}

fn load_from_file(path: &str) -> anyhow::Result<Vec<u8>> {
    Ok(fs::read(path)?)
}
