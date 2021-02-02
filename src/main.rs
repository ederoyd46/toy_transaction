use csv::{ReaderBuilder, Trim};
use std::env;
use std::fs::File;
use std::path::Path;
use std::process;
use toy_transaction::process_transaction_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    let transaction_input = parse_config(&args);
    let transaction_input = parse_csv_reader(transaction_input);

    match process_transaction_file(transaction_input) {
        Ok(()) => eprintln!("Finished"),
        Err(err) => eprintln!("An application error occurred {:#?}", err),
    };
}

fn parse_config(args: &[String]) -> &str {
    if args.len() != 2 {
        eprintln!("Please provide a transaction file to process ONLY");
        process::exit(1);
    };

    &args[1]
}

fn parse_csv_reader(csv_file_location: &str) -> csv::Reader<File> {
    match ReaderBuilder::new()
        .trim(Trim::All)
        .flexible(true)
        .from_path(Path::new(csv_file_location))
    {
        Ok(input) => input,
        Err(err) => {
            eprintln!("Failed to open csv file {:#?}", err);
            process::exit(2)
        }
    }
}
