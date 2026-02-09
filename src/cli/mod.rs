use std::{
    collections::HashMap,
    env, fmt, fs,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use ignore::Walk;
use thiserror::Error;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Specify the directory
    #[arg(short, long)]
    dir: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Clone, Subcommand)]
enum Commands {}

pub fn run() -> Result<(), LocError> {
    let cli = Cli::parse();
    let path = if let Some(specified_path) = cli.dir {
        PathBuf::from(specified_path)
    } else {
        env::current_dir().map_err(|source| LocError::CurrentDirectory { source })?
    };

    count_lines(&path)
}

#[derive(Error, Debug)]
pub enum LocError {
    #[error("failed to determine current working directory")]
    CurrentDirectory {
        #[source]
        source: std::io::Error,
    },
    #[error("failed while traversing directory `{path}`")]
    WalkDirectory {
        path: PathBuf,
        #[source]
        source: ignore::Error,
    },
    #[error("failed to read `{path}`")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum FileType {
    HTML,
    CSS,
    SASS,
    LESS,
    STYLUS,
    PCSS,
    JS,
    TS,
    JSX,
    TSX,
    Vue,
    SVELTE,
    ASTRO,
    RS,
    PY,
    JAVA,
    C,
    H,
    CPP,
    HPP,
    CSHARP,
    GO,
    RB,
    PHP,
    SWIFT,
    KT,
    SCALA,
    LUA,
    R,
    DART,
    ELIXIR,
    ERLANG,
    HS,
    ML,
    MLI,
    FS,
    FSI,
    FSSCRIPT,
    CLJ,
    GROOVY,
    PL,
    PM,
    SH,
    BASH,
    ZSH,
    PS1,
    SQL,
    TOML,
    JSON,
    YAML,
    XML,
    MD,
    MDX,
    DOCKERFILE,
    MAKEFILE,
    Other,
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            FileType::HTML => "HTML",
            FileType::CSS => "CSS",
            FileType::SASS => "Sass",
            FileType::LESS => "Less",
            FileType::STYLUS => "Stylus",
            FileType::PCSS => "PostCSS",
            FileType::JS => "JavasScript",
            FileType::TS => "TypeScript",
            FileType::JSX => "JSX",
            FileType::TSX => "TSX",
            FileType::RS => "Rust",
            FileType::PY => "Python",
            FileType::JAVA => "Java",
            FileType::C => "C",
            FileType::H => "C/C++ Header",
            FileType::CPP => "C++",
            FileType::HPP => "C++ Header",
            FileType::CSHARP => "C#",
            FileType::GO => "Go",
            FileType::RB => "Ruby",
            FileType::PHP => "PHP",
            FileType::SWIFT => "Swift",
            FileType::KT => "Kotlin",
            FileType::SCALA => "Scala",
            FileType::LUA => "Lua",
            FileType::R => "R",
            FileType::DART => "Dart",
            FileType::ELIXIR => "Elixir",
            FileType::ERLANG => "Erlang",
            FileType::HS => "Haskell",
            FileType::ML => "OCaml",
            FileType::MLI => "OCaml Interface",
            FileType::FS => "F#",
            FileType::FSI => "F# Interface",
            FileType::FSSCRIPT => "F# Script",
            FileType::CLJ => "Clojure",
            FileType::GROOVY => "Groovy",
            FileType::PL => "Perl",
            FileType::PM => "Perl Module",
            FileType::SH => "Shell",
            FileType::BASH => "Bash",
            FileType::ZSH => "Zsh",
            FileType::PS1 => "PowerShell",
            FileType::SQL => "SQL",
            FileType::TOML => "Toml",
            FileType::JSON => "JSON",
            FileType::YAML => "YAML",
            FileType::XML => "XML",
            FileType::MD => "Markdown",
            FileType::MDX => "MDX",
            FileType::DOCKERFILE => "Dockerfile",
            FileType::MAKEFILE => "Makefile",
            FileType::Vue => "VueJS",
            FileType::SVELTE => "Svelte",
            FileType::ASTRO => "Astro",
            FileType::Other => "Other",
        };

        write!(f, "{text}")
    }
}

fn count_lines(path: &Path) -> Result<(), LocError> {
    let mut counter: HashMap<FileType, u32> = HashMap::new();
    let mut files: Vec<PathBuf> = vec![];
    for result in Walk::new(path) {
        let entry = result.map_err(|source| LocError::WalkDirectory {
            path: path.to_path_buf(),
            source,
        })?;
        let entry_path = entry.into_path();

        if entry_path.is_file() {
            files.push(entry_path);
        }
    }

    for file in files {
        let file_name = file.file_name().unwrap();
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
            let line_count = text.lines().count() as u32;
            counter
                .entry(file_type)
                .and_modify(|e| *e += line_count)
                .or_insert(line_count);
        }
    }

    println!("{:?}", counter);

    Ok(())
}
