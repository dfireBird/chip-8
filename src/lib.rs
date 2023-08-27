use std::{env, fs, path::PathBuf, time::Duration};

use anyhow::{self, Context};
use minifb::{Key, Scale, Window, WindowOptions};

mod config;
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

pub struct ChipOcto {
    key_map: [Key; 16],
    cycles_per_frame: u32,
}

impl Default for ChipOcto {
    fn default() -> Self {
        Self {
            key_map: KEY_MAP,
            cycles_per_frame: CYCLES_PER_FRAME,
        }
    }
}

impl ChipOcto {
    pub fn create(cycles_per_frame: Option<u32>) -> anyhow::Result<Self> {
        let mut chip_octo = Self {
            ..Default::default()
        };

        let config = load_config();
        if let Some(config) = config {
            if let Some(cycles_per_frame) = config.cycles_per_frame {
                chip_octo.cycles_per_frame = cycles_per_frame;
            }

            if let Some(keys) = config.keys {
                chip_octo.key_map = keys.into_arr();
            }
        }

        if let Some(cycles_per_frame) = cycles_per_frame {
            chip_octo.cycles_per_frame = cycles_per_frame;
        }

        Ok(chip_octo)
    }

    pub fn run(&self, rom_path: &str) -> anyhow::Result<()> {
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
                if let Some(index) = self.key_map.iter().position(|k| *k == key) {
                    input_keys.push(index as u8);
                } else {
                    // Else condition for keys not related to Chip-8, to be used if needed
                }
            }

            for _ in 0..self.cycles_per_frame {
                cpu.step(&mut input_keys);
            }

            cpu.step_timers();

            window
                .update_with_buffer(&cpu.get_framebuffer(), WIDTH, HEIGHT)
                .context("Updating the window")?;
        }

        Ok(())
    }
}

fn load_from_file(path: &str) -> anyhow::Result<Vec<u8>> {
    Ok(fs::read(path)?)
}

fn load_config() -> Option<config::Config> {
    let config_directory_path = dirs::config_dir();
    let config_file_path = config_directory_path
        .unwrap_or(env::current_dir().unwrap_or(PathBuf::from(".")))
        .join("chip-octo.toml");

    let config_file_data = fs::read_to_string(config_file_path);

    if let Ok(config_file_data) = config_file_data {
        let config = toml::from_str(&config_file_data);

        if let Err(e) = &config {
            eprintln!("WARNING: Error parsing the config file. Ignoring config file settings");
            eprintln!("{e}")
        }
        config.ok()
    } else {
        None
    }
}
