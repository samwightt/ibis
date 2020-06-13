mod package;

use clap::{App, AppSettings};
use tokio::join;

/// Creates and runs the main CLI app.
#[tokio::main]
async fn main() -> () {
    let app = App::new("Ibis")
        .version("0.1.0")
        .author("Sam Wight <samuelwight@gmail.com>")
        .about("Ibis is a documentation package manager, viewer, and search engine.")
        .subcommand(package::subcommand())
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    let validate_fut = package::run(&app);
    join!(validate_fut);
}
