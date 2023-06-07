/*
    grep in very little code written in Rust
*/
use clap::Parser;
use anyhow::{Context,Result};
#[derive(Parser)]
struct Cli {
    pattern:String,
    path:std::path::PathBuf,
}
fn main() -> Result<()> {
    let args=Cli::parse();
    let result=std::fs::read_to_string(&args.path).with_context(|| format!("Couldn't read file {}",args.path.display()))?;
    grr::matchp(&result,&args.pattern);
    Ok(())
}