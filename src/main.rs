use clap::Parser;
use lfmt::parser;
use std::fs::read_to_string;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let args: CLIArgs = CLIArgs::parse();
    let str: String = read_to_string(args.path)?;

    let result: String = eval(str)?;
    println!("{}", result);
    Ok(())
}

#[derive(Debug, Parser)]
#[command(version, about, author)]
struct CLIArgs {
    /// A target to format
    path: PathBuf,
}

fn eval(str: String) -> anyhow::Result<String> {
    let result = parser::parse(&str)?;
    todo!();
}
