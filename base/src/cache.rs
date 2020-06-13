//! This module contains all of the file system-related code in Ibis.
//! Any module that interacts with the file system must go through this module first.
//!
//! The purpose of this module is to make sure access to the file system is limited to only
//! the `.ibis` cache directory (or wherever the user specifies using environment variables).
//! All functions in the module are scoped to the `.ibis` directory and can't access anything *above*
//! it. This makes uninstallation easy and limits the amount of malicious activity packages could
//! create.
//!
//! Having all file operations go through this module also means swapping out fs APIs is simple.
//! For instance, right now all functions use the `tokio::fs::File` API, but if we wanted to swap
//! that out with something faster, it could be done very easily.

use std::path::PathBuf;
use anyhow::{Context, Result};
use tokio::{fs::File, fs};
use crate::constants::CACHE_PATH;

/// Utility function for getting the cache path (place where everything in Ibis is stored).
/// Fails if the operating system or user doesn't have a home directory set.
async fn cache_path() -> Result<PathBuf> {
    let mut root = dirs::home_dir().context("Unable to get home directory.")?;
    root.push(CACHE_PATH);
    Ok(root)
}

/// Creates the cache path if it doesn't exist. If it does, just returns the cache
/// path.
pub async fn get_cache_path() -> Result<PathBuf> {
    let path = cache_path().await.context("Could not get path to cache.")?;
    if !path.exists() {
        fs::create_dir(&path).await.context("Could not create cache directory.")?;
        println!(".ibis directory not found. Created .ibis directory.");
    }
    Ok(path)
}

/// Gets a given path starting from the cache root. Fails if the operating system or user
/// doesn't have a home directory set.
pub async fn get_path(p: &str) -> Result<PathBuf> {
    let mut path = get_cache_path().await.context("Could not get cache path.")?;
    path.push(&p);
    Ok(path)
}

/// Creates a directory in the cache directory starting from the cache root.
/// Creates the cache directory if it does not already exist.
pub async fn create_dir(path: &str) -> Result<()> {
    let mut cache_path = get_cache_path().await.context("Could not get cache path.")?;
    cache_path.push(path);
    fs::create_dir(&path).await.context("Could not create directory.")?;
    Ok(())
}

/// Creates a file in the cache directory starting from the cache root.
/// Creates the cache directory if it does not already exist.
/// Returns the `File` struct that is created.
pub async fn create_file(path: &str) -> Result<File> {
    let mut cache_path = get_cache_path().await.context("Could not get cache path.")?;
    cache_path.push(path);
    File::create(cache_path).await.context("Could not create file>")
}

/// 
pub async fn open_file(path: &str) -> Result<File> {
    File::open(get_path(path).await.context("Could not get path")?).await.context("Could not open file")
}