use clap::Parser;
use dashmap::DashMap;
use rayon::prelude::*;
use std::{
    env, fs,
    path::{Path, PathBuf},
    sync::Arc,
};
use tabled::{
    Table,
    settings::{Height, Width, peaker::Priority},
};

use ignore::WalkBuilder;

use crate::types::{cli::Cli, error::LocError, file_count::FileCount, file_type::FileType};

pub fn run() -> Result<(), LocError> {
    let cli = Cli::parse();
    let path = if let Some(specified_path) = cli.dir {
        PathBuf::from(specified_path)
    } else {
        env::current_dir().map_err(|source| LocError::CurrentDirectory { source })?
    };

    let final_count = count_lines(&path)?;

    // builder.push_record(["Language", "Files", "Blank lines", "code", "All lines"]);

    let mut total_files = 0;
    let mut total_blank = 0;
    let mut total_code = 0;

    let mut table_data: Vec<Vec<String>> = vec![vec![
        "Language".into(),
        "Files".into(),
        "Blank lines".into(),
        "code".into(),
        "All lines".into(),
    ]];

    for (file_type, file_count) in final_count {
        let total = file_count.total_files;
        let blank = file_count.blank_lines;
        let loc = file_count.total_loc;
        let code = loc - blank;

        total_files += total;
        total_blank += blank;
        total_code += code;

        table_data.push(vec![
            file_type.to_string(),
            total_files.to_string(),
            total_blank.to_string(),
            total_code.to_string(),
            loc.to_string(),
        ]);
    }

    let total_loc = total_blank + total_code;

    table_data.push(vec![
        "Total".into(),
        total_files.to_string(),
        total_blank.to_string(),
        total_code.to_string(),
        total_loc.to_string(),
    ]);

    let mut table = Table::from_iter(table_data);
    let (width, height) = get_terminal_size();
    table
        .with(Width::wrap(width).priority(Priority::right()))
        .with(Width::increase(width))
        .with(Height::limit(height))
        .with(Height::increase(height));

    println!("{table}");

    Ok(())
}

fn get_terminal_size() -> (usize, usize) {
    let (terminal_size::Width(width), terminal_size::Height(height)) =
        terminal_size::terminal_size().expect("Failed to get the terminal size");

    (width as usize, height as usize)
}

pub fn count_lines(path: &Path) -> Result<DashMap<FileType, FileCount>, LocError> {
    let counter: Arc<DashMap<FileType, FileCount>> = Arc::new(DashMap::new());
    let mut files: Vec<PathBuf> = vec![];
    for entry in WalkBuilder::new(path).build().flatten() {
        let entry_path = entry.into_path();

        if entry_path.is_file() {
            files.push(entry_path);
        }
        // Probably show a warning here?
    }

    files.into_par_iter().for_each(|file| {
        let file_name = file.file_name().unwrap_or_default();
        let file_name_normalized = file_name.to_string_lossy().to_ascii_lowercase();
        let mut file_type = FileType::Other;
        if file_name_normalized == "dockerfile" {
            file_type = FileType::DOCKERFILE;
        } else if file_name_normalized == "makefile" {
            file_type = FileType::MAKEFILE;
        } else if let Some(file_extension) = file_name_normalized.rsplit('.').next() {
            file_type = match file_extension {
                "html" | "xhtml" => FileType::HTML,
                "ts" => FileType::TS,
                "js" => FileType::JS,
                "tsx" => FileType::TSX,
                "jsx" => FileType::JSX,
                "vue" => FileType::Vue,
                "svelte" => FileType::SVELTE,
                "astro" => FileType::ASTRO,
                "css" => FileType::CSS,
                "sass" | "scss" => FileType::SASS,
                "less" => FileType::LESS,
                "styl" | "stylus" => FileType::STYLUS,
                "pcss" => FileType::PCSS,
                "rs" => FileType::RS,
                "py" => FileType::PY,
                "java" => FileType::JAVA,
                "c" => FileType::C,
                "h" => FileType::H,
                "cpp" | "cc" | "cxx" => FileType::CPP,
                "hpp" | "hh" | "hxx" => FileType::HPP,
                "cs" => FileType::CSHARP,
                "go" => FileType::GO,
                "rb" => FileType::RB,
                "php" => FileType::PHP,
                "swift" => FileType::SWIFT,
                "kt" | "kts" => FileType::KT,
                "scala" => FileType::SCALA,
                "lua" => FileType::LUA,
                "r" => FileType::R,
                "dart" => FileType::DART,
                "ex" | "exs" => FileType::ELIXIR,
                "erl" | "hrl" => FileType::ERLANG,
                "hs" => FileType::HS,
                "ml" => FileType::ML,
                "mli" => FileType::MLI,
                "fs" => FileType::FS,
                "fsi" => FileType::FSI,
                "fsx" => FileType::FSSCRIPT,
                "clj" | "cljs" | "cljc" => FileType::CLJ,
                "groovy" | "gvy" | "gy" | "gsh" => FileType::GROOVY,
                "pl" => FileType::PL,
                "pm" => FileType::PM,
                "sh" => FileType::SH,
                "bash" => FileType::BASH,
                "zsh" => FileType::ZSH,
                "ps1" => FileType::PS1,
                "sql" => FileType::SQL,
                "toml" => FileType::TOML,
                "json" => FileType::JSON,
                "yaml" | "yml" => FileType::YAML,
                "xml" => FileType::XML,
                "md" => FileType::MD,
                "mdx" => FileType::MDX,
                _ => FileType::Other,
            }
        };

        if let Result::Ok(text) = fs::read_to_string(&file) {
            let mut blank_lines = 0;
            for line in text.lines() {
                if line.is_empty() {
                    blank_lines += 1;
                }
            }
            let line_count = text.lines().count() as u32;

            counter
                .entry(file_type)
                .and_modify(|e| {
                    e.total_loc += line_count;
                    e.total_files += 1;
                    e.blank_lines += blank_lines
                })
                .or_insert(FileCount {
                    total_files: 1,
                    total_loc: line_count,
                    blank_lines,
                });
        }
    });

    Ok(Arc::try_unwrap(counter).unwrap())
}
