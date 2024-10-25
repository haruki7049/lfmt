use clap::Parser;

fn main() {
    let args: CommandLineArgs = CommandLineArgs::parse();

    println!("The file argument's value: {}", args.file);
    todo!();
}

#[derive(Debug, Parser)]
#[command(version, about, author)]
struct CommandLineArgs {
    /// A target to format
    file: String,
}
