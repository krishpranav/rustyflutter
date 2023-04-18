use std::{fmt::Display, io, path::PathBuf, process::ExitStatus};
use std::path::Path;

// file operations
#[derive(Debug)]
pub enum FileOperation {
    CreateDir,
    Copy,
    Move,
    Remove,
    RemoveDir,
    Read,
    Write,
    Open,
    Create,
    MkDir,
    CopyDir,
    Command,
    Unarchive
}

#[derive(Debug)]
pub enum BuildError {
    ToolError {
        command: String,
        status: ExitStatus,
        stderr: String,
        stdout: String,
    },
    FlutterPathNotFound,
    FlutterPathInvalidError {
        path: PathBuf,
    },
    FlutterLocalEngineNotFound,
    JsonError {
        text: Option<String>,
        source: serde_json::Error,
    },
    OtherError(String),
}

pub type BuildResult<T> = Result<T, BuildError>;

impl Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::ToolError {
                command,
                status,
                stderr,
                stdout,
            } => {
                write!(
                    f,
                    "External tool failed!!"
                )
            }
        }
    }
}

impl std::error::Error for BuildError {}

pub(super) trait IOResultExt<T> {

}