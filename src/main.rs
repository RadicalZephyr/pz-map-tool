use std::path::PathBuf;

use clap::{Args, CommandFactory, Parser};
use pz_map_tool::{Action, MapRegion, Source};

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
    source: SourceOpt,

    /// The action for specified regions of the map
    #[arg(short, long, value_enum)]
    action: Action,

    /// Regions to perform <action> on
    #[arg(short, long, required = true)]
    region: Vec<MapRegion>,
}

#[derive(Clone, Debug, Args)]
#[group(required = true, multiple = false)]
struct SourceOpt {
    /// Specify the save file name
    #[arg(short, long, group = "source")]
    name: Option<String>,

    /// Specify the path to the save file
    #[arg(short, long, group = "source")]
    path: Option<PathBuf>,
}

impl SourceOpt {
    fn into_enum(self) -> Source {
        if let Some(name) = self.name {
            return Source::SaveName(name);
        }
        if let Some(path) = self.path {
            return Source::Path(path);
        }
        // TODO (zefs): It would be cleaner if the Clap API could
        // directly work with the enum representation, but I think
        // this format presents a nicer CLI interface.
        unreachable!("clap options require exactly one of these options to be present");
    }
}

fn main() {
    let cli = Cli::parse();

    if cli.debug > 0 {
        eprintln!("{cli:#?}");
    }

    let Cli {
        debug,
        dry_run,
        source,
        action,
        region,
    } = cli;

    match source.into_enum() {
        Source::SaveName(save_name) => {
            let Some(mut home) = dirs::home_dir() else {
                let mut cmd = Cli::command();
                cmd.error(
                    clap::error::ErrorKind::ValueValidation,
                    "save name specified, but could not find a home directory, please use --path instead."
                ).exit();
            };
            let mut saves_path = home;
            saves_path.extend(["Zomboid", "Saves"]);
        }
        Source::Path(path) => todo!(),
    }
    // Get the Path for the save file by name or direct path
    //   - Verify it's a valid save name/path

    // Create a ModifyMap struct
    //   - Invert Action for `default_action`

    // Get an iterator over all files to be deleted based on ModifyMap
    // spec

    // Iterate over all files and print out/delete files
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
