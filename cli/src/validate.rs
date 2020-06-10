use clap::{App, Arg, ArgMatches};
use colored::Colorize;
use base::{Store, Schema};
use anyhow::{Result, Context};

/// Validates a file against the schema.
async fn validate_schema(file: &str, schema: &mut Schema) -> Result<()> {
    let is_valid = schema
        .validate_file(&file)
        .await
        .context("Could not validate file against schema.")?;
    
    if let Some(errors) = is_valid {
        println!("{}", "Validation failed with errors:".red().bold());
        for error in errors {
            let err_body = &*error.get_title();
            let err_path = &*error.get_path();
            let error_complete = format!("  - {}: {}", err_body.yellow(), err_path);

            println!("{}", error_complete);

            if let Some(e) = error.get_detail() { println!("{}", e) }
        }
    }
    else {
        println!("{}", "Schema validated with no errors!".green().bold());
    }
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

pub async fn run(app: &ArgMatches, store: &mut Store) {
    if let Some(ref verify_command) = app.subcommand_matches("validate") {
        let file = verify_command.value_of("file").unwrap_or("ibis.json");
        let line_to_print = format!("Validating the config of {}...", file);
        println!("{}", line_to_print.blue().bold());
        validate_schema(&file, store.schema()).await.unwrap_or_else(|err| panic!("{}{}", "Error while validating: ".red(), err));
    }
}
