use clap::{ArgAction, Parser, ValueEnum};
use regex::Regex;
use std::path::PathBuf;

mod error;
mod files;
mod parsers;
mod primitives;
mod structs;
pub use crate::error::Error;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Execute,
    Suggest,
    Diff,
}

// Default behavior should be recursive search from calling dir, excluding files in .gitignore
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// -m mode: (E)xecute, (S)uggest, (D)iff using Git
    #[arg(short, long)]
    mode: Mode,

    /// -o override: Pass a json object that overrides defaults. e.g: {"type": "u32", "nat_align": 5, "size": 8}
    #[arg(short, long)]
    overrides: Option<String>,

    /// -i includes: operate on files matching a comma separated list of regex patterns, instead of recursively
    #[arg(short, long)]
    includes: Option<String>,

    /// -e exclude: exclude files matching a comma separated list of regex patterns from recursive search
    #[arg(short, long)]
    exclude: Option<String>,

    /// -d directory: operate from some starting directory, given some relative path to it. e.g: "./src"
    #[arg(short, long, default_value = ".")]
    directory: PathBuf,
}

// Config for conversion of args into easier to use data types,
// input validation, and testability
#[derive(Debug)]
struct Config {
    pub operation_mode: Mode,
    pub overrides: Option<String>,
    pub included_files: Option<Vec<Regex>>,
    pub excluded_files: Option<Vec<Regex>>,
    pub working_directory: PathBuf,
}

impl Config {
    pub fn should_process_file(&self, path: &str) -> bool {
        if let Some(excludes) = &self.excluded_files {
            if excludes.iter().any(|exclude| exclude.is_match(path)) {
                return false;
            }
        }

        if let Some(includes) = &self.included_files {
            if includes.iter().any(|include| include.is_match(path)) {
                return true;
            }
        }

        true
    }
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        // convert include/exclude patterns to Vecs of Regex
        // for usability
        let includes: Option<Vec<Regex>> = args.includes.as_ref().map(|include| {
            include
                .split(',')
                .map(|pattern| Regex::new(pattern).unwrap())
                .collect()
        });
        let excludes: Option<Vec<Regex>> = args.exclude.as_ref().map(|exclude| {
            exclude
                .split(',')
                .map(|pattern| Regex::new(pattern).unwrap())
                .collect()
        });

        Config {
            operation_mode: args.mode,
            overrides: args.overrides,
            included_files: includes,
            excluded_files: excludes,
            working_directory: args.directory,
        }
    }
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let config = Config::from(args);

    dbg!(&config);

    // accumulate analyzable files
    let files = match config.acc_files() {
        Ok(f) => f,
        Err(e) => panic!("analyzing files: {}", e),
    };

    dbg!(&files);

    // analyze files using the CLI mode (execute, suggest, diff)
    config.analyze_files(files);

    // cleanup

    Ok(())
}
