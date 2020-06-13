use serde_json::{from_str, Value};
use valico::json_schema;
use valico::common::error::ValicoErrors;
use crate::cache;
use anyhow::{Result, Context, anyhow};
use tokio::prelude::*;
use tokio::fs::File;
use async_trait::async_trait;

#[async_trait]
pub trait Validator {
    fn path(&self) -> String;
    fn repo_url(&self) -> String;
    /// Attempts to load the schema from the file system, returning a copy of it.
    /// Caches the schema for later use.
    async fn load(&self) -> Result<Value> {
        let file_path = cache::get_path(&self.path()).await?;
        let mut file = File::open(file_path).await.context("Could not open schema.json")?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).await?;
        let schema: Value = from_str(&buffer).context("Failed to parse schema.min.json.")?;
        Ok(schema)
    }

    /// Downloads the schema from the GitHub source.
    /// Fails if the schema isn't accessible. Doesn't continue if the file already exists.
    async fn download(&self) -> Result<()> {
        println!("Could not find {} in cache folder, downloading now...", &self.path());
        let schema_path = cache::get_path(&self.path()).await.context("Could not get path to schema.")?;
        if !schema_path.exists() {
            let result = reqwest::get(&self.repo_url()).await
                .context(format!("Could not get {} from GitHub source.", &self.path()))?
                .text().await
                .context(format!("Could not get {} from GitHub source.", &self.path()))?;
            let mut out = cache::create_file(&self.path()).await.context(format!("Could not create {}", &self.path()))?;
            out.write_all(&result.as_bytes()).await.context(format!("Could not write {}", &self.path()))?;
        }
        else {
            return Err(anyhow!("Tried to download {} but it already existed.", &self.path()));
        }
        println!("Downloaded {} to cache folder.", &self.path());

        Ok(())
    }

    /// Gets the cached version of the schema. Returns Some(schema) if it exists,
    /// or returns None if it does not.
    async fn get(&self) -> Result<Value> {
        if let Ok(schema) = self.load().await {
            Ok(schema)
        }
        else {
            self.download().await.context("Could not download schema.")?;
            Ok(self.load().await.context("Could not load schema from filesystem.")?)
        }
    }

    /// Validates a Serde Value against the Schema to be sure it fits all the required specs.
    /// Returns None if there are no errors, and returns Some(errors) if the JSON isn't valid against the schema.
    /// Calls `load_or_download()` to get the schema.
    /// *Beware the gnarly Valico errors!*
    async fn validate(&self, val: &Value) -> Result<Option<ValicoErrors>> {
        let schema_json = self.get().await?;
        let mut scope: json_schema::Scope = json_schema::Scope::new();

        let schema = scope
            .compile_and_return(schema_json, false).unwrap();
        
        let validate = schema.validate(&val);
        if !validate.is_valid() {
            Ok(Some(validate.errors))
        }
        else {
            Ok(None)
        }
    }

    /// Gets a JSON file, converting it to a Serde Value, then validates it against the schema.
    /// Returns None if there are no errors, and returns Some(errors) if the JSON isn't valid against the schema.
    /// Calls `load_or_download()` to get the schema.
    /// *Beware the gnarly Valico errors!*
    async fn validate_file(&self, file: &str) -> Result<Option<ValicoErrors>> {
        let mut file = File::open(&file).await.context("Could not open file to verify.")?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).await?;
        let to_validate: Value = from_str(&buffer).context("Could not open file to parse.")?;
        self.validate(&to_validate).await
    }
}




