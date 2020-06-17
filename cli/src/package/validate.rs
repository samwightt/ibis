use clap::{App, Arg, ArgMatches};
use colored::Colorize;
use base::validators::{PackageSchema, Validator};
use anyhow::{Result, Context};

/// Validates a file against the schema.
async fn validate_schema(file: &str) -> Result<()> {
    let ps = PackageSchema {};
    ps.validate_file(&file)
        .await
        .context("Could not validate file against schema.")?;

    println!("{}", "Schema validated with no errors!".green().bold());

    Ok(())
}

// The Clap subcommand for the validate module.
pub fn subcommand<'a>() -> App<'a> {
    App::new("validate")
        .about("Validates a ibis.json file. Downloads the schema.json file from GitHub if it's not found.")
        .version("0.1.0")
        .arg(
            Arg::with_name("file")
                .index(1)
                .about("The path to the ibis.json file to verify (default ./ibis.json)."),
        )
}

pub async fn run(app: &ArgMatches) {
    if let Some(ref verify_command) = app.subcommand_matches("validate") {
        let file = verify_command.value_of("file").unwrap_or("ibis.json");
        let line_to_print = format!("Validating the config of {}...", file);
        println!("{}", line_to_print.blue().bold());
        validate_schema(&file).await.unwrap_or_else(|err| println!("{}{:?}", "Error while validating: ".red().bold(), err));
    }
}
