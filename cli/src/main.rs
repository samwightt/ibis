mod validate;

use clap::App;
use futures::join;

#[tokio::main]
async fn main() -> () {
    let app = App::new("Docubus")
        .version("0.1.0")
        .author("Sam Wight <samuelwight@gmail.com>")
        .subcommand(validate::subcommand())
        .get_matches();

    let validate_fut = validate::run(&app);
    join!(validate_fut);
}
