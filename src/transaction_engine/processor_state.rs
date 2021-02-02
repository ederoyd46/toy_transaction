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
        self.processor_state
            .entry(transaction.client)
            .or_insert_with(|| TransactionProcessor::new(transaction.client))
            .add_transaction(transaction);
    }

    pub fn get_state(&self) -> &HashMap<u16, TransactionProcessor> {
        &self.processor_state
    }
}
