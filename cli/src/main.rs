mod validate;

use clap::{App, AppSettings};
use futures::join;

/// Creates and runs the main CLI app.
#[tokio::main]
async fn main() -> () {
    let app = App::new("Ibis")
        .version("0.1.0")
        .author("Sam Wight <samuelwight@gmail.com>")
        .about("Ibis is a documentation package manager, viewer, and search engine.")
        .subcommand(validate::subcommand())
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    let validate_fut = validate::run(&app);
    join!(validate_fut);
}
