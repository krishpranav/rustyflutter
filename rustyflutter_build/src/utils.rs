use crate::{error::IOResultExt, BuildError, BuildResult, FileOperation};
use std::{
    env,
    fs::{self, canonicalize},
    path::{Path, PathBuf},
    process::Command,
};

pub(super) fn get_artifacts_dir() -> BuildResult<PathBuf> {
    let out_dir: PathBuf = std::env::var("OUT_DIR").unwrap().into();
    let artifacts_dir = out_dir.join("../../../");
    let artifacts_dir = artifacts_dir
        .canonicalize()
        .wrap_error(FileOperation::Canonicalize, || artifacts_dir)?;
    Ok(artifacts_dir)
}

pub(super) fn get_absolute_path<P: AsRef<Path>>(path: P) -> PathBuf {
    if path.as_ref().is_absolute() {
        path.as_ref().into()
    } else {
        let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let project_path: PathBuf = manifest.into();
        project_path.join(path)
    }
}

pub(super) fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> BuildResult<()> {
    #[cfg(target_family = "windows")]
    {}

    #[cfg(target_family = "unix")]
    {}
}

pub(super) fn mkdir<P, Q>(target_path: P, sub_path: Option<Q>) -> BuildResult<PathBuf>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let target = match sub_path {
        Some(sub_path) => target_path.as_ref().join(sub_path.as_ref()),
        None => target_path.as_ref().into(),
    };

    fs::create_dir_all(&target).wrap_error(FileOperation::MkDir, || target.clone())?;
    Ok(target)
}
