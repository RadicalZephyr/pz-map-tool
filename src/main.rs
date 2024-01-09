use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn main() {
    let args = Args::parse();

    println!("{args:#?}");
}
