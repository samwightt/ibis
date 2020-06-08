use clap::{App, Arg};
use colored::*;
use serde_json::{from_reader, Value};
use std::fs::File;
use valico::json_schema;

fn validate_schema(file: &String) {
    println!("{:?}", std::env::current_dir());
    let json_v4_schema: Value =
        from_reader(File::open("../schema/schema.json").expect("Couldn't open schema."))
            .expect("Couldn't serialize JSON.");
    let to_validate: Value = from_reader(File::open(&file).expect("Couldn't open file."))
        .expect("Couldn't serialize JSON.");

    let mut scope = json_schema::Scope::new();
    let schema = scope
        .compile_and_return(json_v4_schema.clone(), false)
        .unwrap();

    let validate = schema.validate(&to_validate);
    if !validate.is_valid() {
        println!("{}", "Validation failed with errors:".red().bold());
        for error in validate.errors {
            let err_body = &*error.get_title();
            let err_path = &*error.get_path();
            let error_complete = format!("  - {}: {}", err_body.yellow(), err_path);

            println!("{}", error_complete);

            match error.get_detail() {
                Some(e) => println!("{}", &e),
                None => {}
            }
        }
    }
}

fn main() {
    let app = App::new("Docubus")
        .version("0.1.0")
        .author("Sam Wight <samuelwight@gmail.com>")
        .subcommand(
            App::new("validate")
                .about("Validates a docubus.json file.")
                .version("0.1.0")
                .arg(Arg::with_name("file").index(1).about(
                    "The path to the docubus.json file to verify (default ./docubus.json).",
                )),
        )
        .get_matches();

    if let Some(ref verify_command) = app.subcommand_matches("validate") {
        let mut file = "docubus.json";
        if verify_command.is_present("file") {
            println!(
                "Setting to the value of {}.",
                verify_command.value_of("file").unwrap()
            );
            file = verify_command
                .value_of("file")
                .expect("There was an error getting the file to load.");
        }
        let line_to_print = format!("Validating the config of {}...", file);
        println!("{}", line_to_print.blue().bold());
        validate_schema(&String::from(file));
    }
}
