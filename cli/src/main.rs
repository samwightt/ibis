mod validate;

use clap::{App, AppSettings};
use futures::join;
use base::Store;

/// Creates and runs the main CLI app.
#[tokio::main]
async fn main() -> () {
    let app = App::new("Docubus")
        .version("0.1.0")
        .author("Sam Wight <samuelwight@gmail.com>")
        .about("Docubus is a documentation package manager, viewer, and search engine.")
        .subcommand(validate::subcommand())
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    // Create the root store.
    let mut store = Store::new()
        .expect("Error: Could not create .docubus directory. Please make sure you have permissions to do so.");

    let validate_fut = validate::run(&app, &mut store);
    join!(validate_fut);
}
