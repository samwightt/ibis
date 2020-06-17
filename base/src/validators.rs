mod validator;
pub use validator::Validator;
use crate::constants;
use async_trait::async_trait;
use anyhow::{Result, anyhow};
use serde_json::Value;
use std::collections::HashSet;

pub struct PackageSchema {}

#[async_trait]
impl Validator for PackageSchema {
  fn path(&self) -> String {
    String::from(constants::SCHEMA_PATH)
  }

  fn repo_url(&self) -> String {
    String::from(constants::SCHEMA_REPO_PATH)
  }
  
  async fn after_validate(&self, val: &Value) -> Result<()> {
    // I don't know what I was thinking when I wrote this... This doesn't take into account no children or
    // the fact that there are multiple pages in a json file???? Seriously don't know what was in my head.
    // TODO: Fix child validation. There are more pressing issues now though.

    // let pages = val.get("pages").ok_or(anyhow!("Could not get pages."))?;
    // let mut page_ids: HashSet<String> = HashSet::new();

    // for (_, val) in pages.as_object().ok_or(anyhow!("Could not get keys in pages."))? {
    //   page_ids.insert(val.to_string());
    // }

    // let children = pages.get("children").ok_or(anyhow!("Could not get children."))?.as_array().unwrap();

    // for child in children {
    //   if !page_ids.contains(&child.to_string()) { return Err(anyhow!("Error finding child.")) }
    // }

    Ok(())
  }
}

pub struct RepoSchema {}

#[async_trait]
impl Validator for RepoSchema {
  fn path(&self) -> String {
    String::from(constants::REPO_PATH)
  }

  fn repo_url(&self) -> String {
    String::from(constants::REPO_REPO_PATH)
  }

  async fn after_validate(&self, val: &Value) -> Result<()> {
    Ok(())
  }
}