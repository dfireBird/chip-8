fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    chip_octo::run(&args[1])?;
    Ok(())
}
