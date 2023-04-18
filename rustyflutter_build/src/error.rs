use std::{fmt::Display, io, path::PathBuf, process::ExitStatus};

// NOTE: file operations
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
    SymLink,
    MetaData,
    CopyDir,
    MkDir,
    ReadDir,
    Canonicalize,
    Command,
    Unarchive,
}

// build error msg composing
#[derive(Debug)]
pub enum BuildError {
    ToolError {
        command: String,
        status: ExitStatus,
        stderr: String,
        stdout: String,
    },
    FlutterNotFoundError,
    FlutterPathInvalidError {
        path: PathBuf,
    },
    FlutterLocalEngineNotFound,
    FileOperationError {
        operation: FileOperation,
        path: PathBuf,
        source_path: Option<PathBuf>,
        source: io::Error,
    },
    JsonError {
        text: Option<String>,
        source: serde_json::Error,
    },
    YamlError {
        source: yaml_rust::ScanError,
    },
    OtherError(String),
}

// build result type
pub type BuildResult<T> = Result<T, BuildError>;

// displaying build errors
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
                    "Tool Failed\nTool Status: {status:?}\nCommand: {command}\nStderr:\n{stderr}\nStdout:\n{stdout}"
                )
            }
            BuildError::FileOperationError {
                operation,
                path,
                source_path,
                source,
            } => match source_path {
                Some(source_path) => {
                    write!(
                        f,
                        "File operation failed: {operation:?}, target path: {path:?}, source path: {source_path:?}, error: {source}"
                    )
                }
                None => {
                    write!(
                        f,
                        "File operation failed: {operation:?}, path: {path:?}, error: {source}"
                    )
                }
            },
            BuildError::JsonError { text, source } => {
                write!(f, "JSON operation failed: ${source}")?;
                if let Some(text) = text {
                    write!(f, "Text:\n{text}")?;
                }
                Ok(())
            }
            BuildError::YamlError { source } => {
                write!(f, "{source}")
            }
            BuildError::OtherError(err) => {
                write!(f, "{err}")
            }
            BuildError::FlutterNotFoundError => {
                write!(
                    f,
                    "Can't able to find flutter \
                    Plase make sure 'flutter' executable is in PATH"
                )
            }
            BuildError::FlutterPathInvalidError { path } => {
                write!(
                    f,
                    "Flutter path {path:?} doesn't seem to be a valid flutter installation"
                )
            }
            BuildError::FlutterLocalEngineNotFound => {
                write!(
                    f,
                    "Could not able to find the flutter engine please specify the correct path"
                )
            }
        }
    }
}

impl std::error::Error for BuildError {}

pub(super) trait IOResultExt<T> {
    fn wrap_error<F>(self, operation: FileOperation, path: F) -> BuildResult<T>
        where
            F: FnOnce() -> PathBuf;
    fn wrap_error_with_src<F, G>(
        self,
        operation: FileOperation,
        path: F,
        source_path: G,
    ) -> BuildResult<T>
        where
            F: FnOnce() -> PathBuf,
            G: FnOnce() -> PathBuf;
}

impl<T> IOResultExt<T> for io::Result<T> {
    fn wrap_error<F>(self, operation: FileOperation, path: F) -> BuildResult<T>
        where
            F: FnOnce() -> PathBuf,
    {
        self.map_err(|e| BuildError::FileOperationError {
            operation,
            path: path(),
            source_path: None,
            source: e,
        })
    }

    fn wrap_error_with_src<F, G>(
        self,
        operation: FileOperation,
        path: F,
        source_path: G,
    ) -> BuildResult<T>
        where
            F: FnOnce() -> PathBuf,
            G: FnOnce() -> PathBuf,
    {
        self.map_err(|e| BuildError::FileOperationError {
            operation,
            path: path(),
            source_path: Some(source_path()),
            source: e,
        })
    }
}