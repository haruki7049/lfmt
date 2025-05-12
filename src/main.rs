use clap::Parser;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let args: CLIArgs = CLIArgs::parse();

    dbg!(args.path);
    Ok(())
}

#[derive(Debug, Parser)]
#[command(version, about, author)]
struct CLIArgs {
    /// A target to format
    path: PathBuf,
}
