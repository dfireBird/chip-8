fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    chip8::run(&args[1])?;
    Ok(())
}
