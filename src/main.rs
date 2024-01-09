use std::path::PathBuf;

use clap::{Args, Parser};
use pz_map_tool::Action;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// Print which files would be removed without doing anything
    #[arg(long)]
    dry_run: bool,

    #[command(flatten)]
    source: Source,

    /// The action for specified regions of the map
    #[arg(short, long, value_enum)]
    action: Action,
}

#[derive(Clone, Debug, Args)]
#[group(required = true, multiple = false)]
struct Source {
    /// Specify the save file name
    #[arg(short, long, group = "source")]
    name: Option<String>,

    /// Specify the path to the save file
    #[arg(short, long, group = "source")]
    path: Option<PathBuf>,
}

fn main() {
    let args = Cli::parse();

    println!("{args:#?}");
}
