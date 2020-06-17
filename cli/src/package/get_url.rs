use clap::{App, Arg, ArgMatches};
use colored::Colorize;
use base::packages::get_package_url;
use anyhow::{Result, Context};

// The Clap subcommand for the validate module.
pub fn subcommand<'a>() -> App<'a> {
    App::new("get-url")
        .about("Gets the repo url for a given package name.")
        .version("0.1.0")
        .arg(
            Arg::with_name("name")
                .index(1)
                .about("The package name to get the URL for."),
        )
}

pub async fn get_url(name: &str) {
    println!("{}{}{}", "Getting the URL of ".blue().bold(), name.blue().bold(), "...".blue().bold());
    match get_package_url(&String::from(name)).await {
        Ok(result) => println!("{}{}", "Package repo URL: ".green().bold(), result),
        Err(e) => println!("{:?}", e)
    }
}

pub async fn run(app: &ArgMatches) {
    if let Some(ref get_url_command) = app.subcommand_matches("get-url") {
        let file = get_url_command.value_of("name");
        match file {
            Some(result) => get_url(result).await,
            None => println!("{}", "Please enter a package name to get the URL of.".red().bold())
        }
    }
}
