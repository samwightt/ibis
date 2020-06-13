mod validator;
pub use validator::Validator;
use crate::constants;

pub struct PackageSchema {}

impl Validator for PackageSchema {
  fn path(&self) -> String {
    String::from(constants::SCHEMA_PATH)
  }

  fn repo_url(&self) -> String {
    String::from(constants::SCHEMA_REPO_PATH)
  }
}

pub struct RepoSchema {}

impl Validator for RepoSchema {
  fn path(&self) -> String {
    String::from(constants::REPO_PATH)
  }

  fn repo_url(&self) -> String {
    String::from(constants::REPO_REPO_PATH)
  }
}