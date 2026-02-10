use std::path::PathBuf;

use thiserror::Error;

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
