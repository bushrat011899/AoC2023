use clap::Parser;

/// Command arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file from AoC
    #[arg(short, long, default_value_t = String::from("inputs/day_1.txt"))]
    input: String,
}

fn main() {
    let args = Args::parse();

    let _input = std::fs::read_to_string(args.input).expect("must be able to read input file");

    println!("Parsed");
}
