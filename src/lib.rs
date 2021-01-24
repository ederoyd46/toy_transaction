mod transaction_engine;
use csv::{ReaderBuilder, Trim};
use std::io;
use std::path::Path;
use transaction_engine::{
    TransactionEngineProcessorState, TransactionInput, TransactionRunningState,
};

type Error = Box<dyn std::error::Error>;

pub fn process_transaction_file(transaction_file: &str) -> Result<(), Error> {
    // TODO: Check - According to the docs the file should be streamed so no need to wrap in a buffered reader
    let mut reader = ReaderBuilder::new()
        .trim(Trim::All)
        .flexible(true)
        .from_path(Path::new(transaction_file))?;

    let mut transaction_processor_state = TransactionEngineProcessorState::new();

    for input in reader.deserialize() {
        let transaction: TransactionInput = input?;
        transaction_processor_state.add_transaction(transaction);
    }

    // TODO Probably more memory efficient to do this in a for loop and write out directly rather than process all at once
    let transaction_output: Vec<TransactionRunningState> = transaction_processor_state
        .get_state()
        .iter()
        .map(|(_, transaction_processor)| transaction_processor.process_transactions())
        .collect();
    write_to_output(transaction_output)?;

    Ok(())
}

fn write_to_output(transaction_output: Vec<TransactionRunningState>) -> Result<(), Error> {
    let mut csv_writer = csv::Writer::from_writer(io::stdout());
    for output in transaction_output {
        csv_writer.serialize(&output)?;
    }

    Ok(())
}
