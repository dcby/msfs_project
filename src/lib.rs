mod config;
mod ops;

use std::{
    error::Error,
    fmt::Debug,
    fs::{self},
    io::ErrorKind,
    path::PathBuf,
};

pub use config::Config;

use crate::config::{AppConfig, ProjectConfig};

pub fn create_msfs_project<C>(config: &C) -> Result<(), Box<dyn Error>>
where
    C: AppConfig + ProjectConfig + Debug,
{
    let mut path_buf = PathBuf::new();
    if let Some(p) = config.path() {
        path_buf.push(p);
    }

    path_buf.push(config.slug());

    if config.is_force() {
        fs::remove_dir_all(&path_buf).or_else(|e| match e.kind() {
            ErrorKind::NotFound => Ok(()),
            _ => Err(e),
        })?;
    }

    // create directory
    fs::create_dir(&path_buf)?;

    // project
    ops::create_project(&path_buf, config)?;

    ops::create_package_definitions(&path_buf, config)?;
    ops::create_package_sources(&path_buf)?;

    Ok(())
}
