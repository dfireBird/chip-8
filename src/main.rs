use chip_octo::ChipOcto;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let chip_octo = ChipOcto::default();
    chip_octo.run(&args[1])?;
    Ok(())
}
