use git2::Repository;
use anyhow::{Result, Context, anyhow};
use serde_json::{Value, Result as SResult};

/// Gets the git repository URL of a given package name. Uses
/// the `https://github.com/samwightt/ibis-repo` repo by default.
pub async fn get_package_url(package_name: &String) -> Result<String> {
    let url = format!("https://raw.githubusercontent.com/samwightt/ibis-repo/main/packages/{}.json", package_name);
    let body = reqwest::get(&url)
        .await.context("Failed to get URL.")?
        .text()
        .await.context("Failed to get body.")?;
    
    let result = parse_listing(&body).context("Could not parse JSON.")?;

    let url = result.get("repo").ok_or(anyhow!("Could not find repo."))?;

    if let Value::String(git_url) = url {
        Ok(git_url.clone())
    }
    else {
        Err(anyhow!("Could not find repo URL."))
    }
}

fn parse_listing(listing: &String) -> SResult<Value> {
    serde_json::from_str(&listing)
}