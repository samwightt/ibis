pub mod schema;

use anyhow::{Context, Result};
use schema::Schema;
use std::path::PathBuf;

/// Represents the root store instance. The store (as of right now) is located at
/// home_dir/.docubus. Documentation packages, schema files, index files, etc. are
/// all stored at this directory.
pub struct Store {
    schema: Schema,
    cache_path: PathBuf,
}

impl Store {
    /// Utility function for getting the cache path (place where everything in Docubus is stored).
    /// Fails if the operating system or user doesn't have a home directory set.
    pub fn get_cache_path() -> Result<PathBuf> {
        let mut root = dirs::home_dir().context("Unable to get home directory.")?;
        root.push(".docubus");
        Ok(root)
    }

    /// Returns the schema as a mutable reference. Needed for most operations on the schema.
    pub fn schema(&mut self) -> &mut Schema {
        &mut self.schema
    }

    /// Gets a given path, starting from the `cache_path` root.
    pub fn get_path(&self, p: &str) -> Result<PathBuf> {
        let mut path = PathBuf::from(&self.cache_path);
        path.push(&p);
        Ok(path)
    }

    /// Creates a new Store instance. Creates a cache directory if one does not exist yet.
    /// The directory isn't populated with any content until later methods are called.
    pub fn new() -> Result<Self> {
        let cache_path = Store::get_cache_path()?;
        if !cache_path.exists() {
            std::fs::create_dir(&cache_path).context("Could not create cache directory.")?;
            println!(".docubus cache directory not found. Created .docubus directory.");
        }

        let mut schema_path = PathBuf::from(&cache_path);
        schema_path.push("schema.min.json");

        Ok(Store {
            schema: Schema::new(schema_path),
            cache_path,
        })
    }
}
