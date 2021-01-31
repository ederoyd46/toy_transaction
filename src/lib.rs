mod transaction_engine;
use std::error;
use std::io;
use transaction_engine::{TransactionEngineProcessorState, TransactionInput};

pub type Error = Box<dyn error::Error + Sync + Send>;

pub fn process_transaction_file<T: io::Read>(mut reader: csv::Reader<T>) -> Result<(), Error> {
    let mut transaction_processor_state = TransactionEngineProcessorState::new();

    for input in reader.deserialize() {
        let transaction: TransactionInput = input?;
        transaction_processor_state.add_transaction(transaction);
    }

    let mut csv_writer = csv::Writer::from_writer(io::stdout());
    transaction_processor_state
        .get_state()
        .iter()
        .map(|(_, transaction_processor)| transaction_processor.process_transactions())
        .try_for_each(|t| csv_writer.serialize(t))?;

    Ok(())
}
