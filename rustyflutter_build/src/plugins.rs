use std::{
    collections::HashMap,
    fs::{self},
    path::{Path, PathBuf}
};

use yaml_rust::YamlLoader;

use crate::{
    artifacts_emitter::ArtifactsEmitter, BuildError, BuildResult, FileOperation, Flutter,
    IOResultExt, TargetOS,
};

#[cfg(target_os = "macos")]
#[path = "plugin_macos.rs"]
mod plugins_impl;

#[cfg(target_os = "windows")]
#[path = "plugin_windows.rs"]
mod plugins_impl;

#[cfg(target_os = "macos")]
#[path = "plugin_linux.rs"]
mod plugins_impl;

#[derive(Debug)]
pub(crate) struct PluginPlatformInfo {
    pub plugin_class: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct Plugin {
    pub name: String,
    pub path: PathBuf,
    pub platform_path: PathBuf,
    pub platform_name: String,
    pub platform_info: PluginPlatformInfo,
}

pub(super) struct Plugins<'a> {
    build: &'a Flutter<'a>,
    artifacts_emitter: &'a ArtifactsEmitters<'a>
}