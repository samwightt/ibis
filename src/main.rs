use clap::{Arg, App};
use colored::*;

fn main() {
    let app = App::new("Docubus")
        .version("0.1.0")
        .author("Sam Wight <samuelwight@gmail.com>")
        .subcommand(App::new("verify")
            .about("Verifies a docubus.json file.")
            .version("0.1.0")
            .arg(Arg::new("file")
                .index(1)
                .about("The path to the docubus.json file to verify (default ./docubus.json)."))).get_matches();

    if let Some(ref verify_command) = app.subcommand_matches("verify") {
        let mut file = "docubus.json";
        if verify_command.is_present("file") {
            file = verify_command.value_of("file").expect("There was an error getting the file to load.");
        }
        let line_to_print = format!("Verifying the config of {}...", file);
        println!("{}", line_to_print.blue().bold());
    }
}
