use std::env;
use std::process;

use toy_transaction::process_transaction_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    let transaction_input = parse_config(&args);

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
