use clap::{ArgAction, Parser, ValueEnum};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::LazyLock,
};

mod error;
mod parsers;
pub use crate::error::Error;

enum Language {
    Rust,
    Go,
    C,
    Cpp,
}

fn detect_language(file_path: &str) -> Option<Language> {
    let extension = Path::new(file_path).extension().and_then(|e| e.to_str());

    match extension {
        Some("rs") => Some(Language::Rust),
        Some("go") => Some(Language::Go),
        Some("c") => Some(Language::C),
        Some("cpp") => Some(Language::Cpp),
        _ => None,
    }
}

#[derive(Debug, Clone)]
struct TypeInfo {
    nat_align: u8,
    size: u8,
}

#[rustfmt::skip]
static TYPE_INFO: LazyLock<HashMap<&'static str, TypeInfo>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    // Integers
    m.insert("u8", TypeInfo { nat_align: 1, size: 1 });
    m.insert("i8", TypeInfo { nat_align: 1, size: 1 });
    m.insert("u16", TypeInfo { nat_align: 2, size: 2 });
    m.insert("i16", TypeInfo { nat_align: 2, size: 2 });
    m.insert("u32", TypeInfo { nat_align: 4, size: 4 });
    m.insert("i32", TypeInfo { nat_align: 4, size: 4 });
    m.insert("u64", TypeInfo { nat_align: 8, size: 8 });
    m.insert("i64", TypeInfo { nat_align: 8, size: 8 });
    // Floating point
    m.insert("f32", TypeInfo { nat_align: 4, size: 4 });
    m.insert("f64", TypeInfo { nat_align: 8, size: 8 });
    // Others
    m.insert("bool", TypeInfo { nat_align: 1, size: 1 });
    m.insert("char", TypeInfo { nat_align: 4, size: 4 });
    m
});

fn load_gitignore() -> Result<HashSet<String>, std::io::Error> {
    let content = std::fs::read_to_string("./gitignore")?;
    let ignores = content
        .split('\n')
        .filter(|l| !l.is_empty() && !l.contains('#'))
        .map(|l| l.to_string())
        .collect();
    Ok(ignores)
}

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
    /// -a all: search all files recursively from this dir, including files in .gitignore
    #[arg(short, long, action = ArgAction::SetTrue)]
    all: bool,

    /// -op operation: (E)xecute, (S)uggest, (D)iff using Git
    #[arg(short, long)]
    mode: Mode,

    /// -ov override: Pass a json object that overrides defaults. e.g: {"type": "u32", "nat_align": 5, "size": 8}
    #[arg(short, long)]
    overrides: Option<String>,

    /// -f files: operate on a comma separated list (or single) files, instead of recursively
    #[arg(short, long)]
    files: Option<String>,

    /// -e exclude: exclude a comma separated list of files from recursive search
    #[arg(short, long)]
    exclude: Option<String>,

    /// -d directory: operate from some starting directory, given some relative path to it. e.g: "./src"
    #[arg(short, long)]
    directory: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let all = args.all;
    let mode = args.mode;
    let overrides = args.overrides;
    let files = args.files;
    let exclude = args.exclude;
    let directory = args.directory;

    let ignores = match load_gitignore() {
        Ok(i) => i,
        Err(_) => {
            eprintln!("error loading .gitignore file. continuing without");
            HashSet::new()
        }
    };

    Ok(())
}
