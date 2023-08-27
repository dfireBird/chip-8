use clap::Parser;

use chip_octo::ChipOcto;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    rom_file_path: String,

    #[arg(short, long)]
    cycles_per_frame: Option<u32>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let chip_octo = ChipOcto::create(args.cycles_per_frame)?;
    chip_octo.run(&args.rom_file_path)?;
    Ok(())
}
