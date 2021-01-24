use super::{TransactionInput, TransactionProcessor};
use std::collections::HashMap;

#[derive(Debug)]
pub struct TransactionEngineProcessorState {
    processor_state: HashMap<u16, TransactionProcessor>,
}

impl TransactionEngineProcessorState {
    pub fn new() -> Self {
        TransactionEngineProcessorState {
            processor_state: HashMap::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: TransactionInput) {
        //TODO Change this to a match?
        if !self.processor_state.contains_key(&transaction.client) {
            let client_account = TransactionProcessor::new(transaction.client);
            self.processor_state
                .insert(client_account.client, client_account);
        }
        // This should always be Some now
        if let Some(processor) = self.processor_state.get_mut(&transaction.client) {
            processor.add_transaction(transaction);
        }
    }

    pub fn get_state(&self) -> &HashMap<u16, TransactionProcessor> {
        &self.processor_state
    }
}
