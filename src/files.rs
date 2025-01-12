use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::{
    parsers::{base::StructParser, rust::RustParser},
    Config, Error,
};

pub enum Language {
    Rust,
    Go,
    C,
    Cpp,
}

fn detect_operable_language(file_path: &str) -> Option<Language> {
    let extension = Path::new(file_path).extension().and_then(|e| e.to_str());
    match extension {
        Some("rs") => Some(Language::Rust),
        Some("go") => Some(Language::Go),
        Some("c") => Some(Language::C),
        Some("cpp") => Some(Language::Cpp),
        _ => None,
    }
}

impl Config {
    pub fn accumulate_files(&self) -> Result<Vec<PathBuf>, Error> {
        let mut files = Vec::new();

        for entry in WalkDir::new(&self.working_directory)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if let Some(path_str) = path.to_str() {
                if detect_operable_language(path_str).is_none() {
                    continue;
                }
                if self.should_process_file(path_str) {
                    files.push(path.to_path_buf())
                }
            }
        }

        Ok(files)
    }

    pub fn analyze_files(&self, files: Vec<PathBuf>) -> Result<(), Error> {
        // collect structs across all files so we can map relationships
        let mut structs = Vec::new();

        for file in files {
            // determine correct language parser to use
            let extension = file.extension().and_then(|e| e.to_str());
            let parser = match extension {
                Some("rs") => RustParser::new(),
                _ => panic!("unimplemented or unsupported language"),
            };

            // FIXME: use BufReader since we now have a .lines() inner call
            let file_content = match std::fs::read_to_string(&file) {
                Ok(c) => c,
                Err(e) => panic!("reading file content: {}", e),
            };

            // parse struct(s) in the file
            let mut parsed_structs = match parser.parse(file_content) {
                Ok(s) => s,
                Err(e) => panic!("parsing structs from file content: {}", e),
            };

            structs.append(&mut parsed_structs);
        }

        dbg!(&structs);

        // map the parsed primitive data types and calculate memory layout

        // optimization algorithm

        // pretty-print, implement, or git diff depending on CLI mode

        Ok(())
    }
}
