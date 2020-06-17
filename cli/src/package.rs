mod validate;
mod get_url;

use clap::{App, ArgMatches};
use tokio::join;

// The Clap subcommand for the validate module.
pub fn subcommand<'a>() -> App<'a> {
    App::new("package")
        .about("Commands related to individual packages.")
        .version("0.1.0")
        .subcommand(validate::subcommand())
        .subcommand(get_url::subcommand())
}

pub async fn run(app: &ArgMatches) {
    if let Some(ref verify_command) = app.subcommand_matches("package") {
        let v = validate::run(&verify_command);
        let g = get_url::run(&verify_command);
        join!(v, g);
    }
}