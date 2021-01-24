use super::TransactionInput;
use super::TransactionRunningState;
use super::TransactionType;

#[derive(Debug)]
pub struct TransactionProcessor {
    pub client: u16,
    transactions: Vec<TransactionInput>,
}

impl TransactionProcessor {
    pub fn new(client: u16) -> Self {
        TransactionProcessor {
            client,
            transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: TransactionInput) {
        self.transactions.push(transaction);
    }

    pub fn process_transactions(&self) -> TransactionRunningState {
        let mut running_state = TransactionRunningState::new(self.client);

        for transaction in &self.transactions {
            self.process_transaction(&transaction, &mut running_state)
        }

        running_state
    }

    fn process_transaction(
        &self,
        transaction: &TransactionInput,
        transaction_state: &mut TransactionRunningState,
    ) {
        match &transaction.transaction_type {
            TransactionType::Deposit => {
                if let Some(amount) = transaction.amount {
                    transaction_state.deposit(amount);
                }
            }
            TransactionType::Withdrawal => {
                if let Some(amount) = transaction.amount {
                    transaction_state.withdraw(amount);
                }
            }
            TransactionType::Dispute => {
                if let Some(existing_transaction) = self.find_transaction(transaction.tx) {
                    transaction_state.hold(existing_transaction.amount.unwrap())
                }
            }
            TransactionType::Resolve => {
                if let Some(existing_transaction) = self.find_transaction(transaction.tx) {
                    transaction_state.release(existing_transaction.amount.unwrap())
                }
            }
            TransactionType::Chargeback => {
                if let Some(existing_transaction) = self.find_transaction(transaction.tx) {
                    match existing_transaction.transaction_type {
                        TransactionType::Deposit => transaction_state
                            .chargeback_deposit(existing_transaction.amount.unwrap()),
                        TransactionType::Withdrawal => transaction_state
                            .chargeback_withdrawal(existing_transaction.amount.unwrap()),
                        _ => eprintln!("Tried to charge back an invalid transaction type"),
                    }
                }
            }
        }
    }

    fn find_transaction(&self, tx: u32) -> Option<&TransactionInput> {
        self.transactions
            .iter()
            .filter(|transaction| {
                TransactionType::Deposit == transaction.transaction_type
                    || TransactionType::Withdrawal == transaction.transaction_type
            })
            .find(|transaction| transaction.tx == tx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_deposit_transaction() {
        let mut test_obj = TransactionProcessor::new(1);
        let deposit_transaction = TransactionInput {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(1.1111_f64),
        };

        test_obj.add_transaction(deposit_transaction);

        let process_transactions = test_obj.process_transactions();
        assert_eq!(process_transactions.total, 1.1111_f64);
        assert_eq!(process_transactions.held, 0.0_f64);
        assert_eq!(process_transactions.available, 1.1111_f64);
        assert_eq!(process_transactions.locked, false);
    }

    #[test]
    fn handle_deposit_and_withdrawal() {
        let mut test_obj = TransactionProcessor::new(1);
        let deposit_transaction = TransactionInput {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(1.1111_f64),
        };
        let withdrawal_transaction = TransactionInput {
            transaction_type: TransactionType::Withdrawal,
            client: 1,
            tx: 2,
            amount: Some(0.1111_f64),
        };

        test_obj.add_transaction(deposit_transaction);
        test_obj.add_transaction(withdrawal_transaction);
        let process_transactions = test_obj.process_transactions();
        assert_eq!(process_transactions.total, 1.000_f64);
        assert_eq!(process_transactions.held, 0.0_f64);
        assert_eq!(process_transactions.available, 1.000_f64);
        assert_eq!(process_transactions.locked, false);
    }

    #[test]
    fn handle_ignore_withdraw_more_than_available_funds() {
        let mut test_obj = TransactionProcessor::new(1);
        let deposit_transaction = TransactionInput {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(1.0_f64),
        };
        let withdrawal_transaction = TransactionInput {
            transaction_type: TransactionType::Withdrawal,
            client: 1,
            tx: 2,
            amount: Some(2.0_f64),
        };

        test_obj.add_transaction(deposit_transaction);
        test_obj.add_transaction(withdrawal_transaction);
        let process_transactions = test_obj.process_transactions();
        assert_eq!(process_transactions.total, 1.000_f64);
        assert_eq!(process_transactions.held, 0.0_f64);
        assert_eq!(process_transactions.available, 1.000_f64);
        assert_eq!(process_transactions.locked, false);
    }

    #[test]
    fn handle_dispute_amount() {
        let mut test_obj = TransactionProcessor::new(1);
        let deposit_transaction_1 = TransactionInput {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(100.0),
        };
        let deposit_transaction_2 = TransactionInput {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 2,
            amount: Some(50.0),
        };
        let disputed_transaction = TransactionInput {
            transaction_type: TransactionType::Dispute,
            client: 1,
            tx: 2,
            amount: None,
        };

        test_obj.add_transaction(deposit_transaction_1);
        test_obj.add_transaction(deposit_transaction_2);
        test_obj.add_transaction(disputed_transaction);
        let process_transactions = test_obj.process_transactions();
        assert_eq!(process_transactions.total, 150.0_f64);
        assert_eq!(process_transactions.held, 50.0_f64);
        assert_eq!(process_transactions.available, 100.0_f64);
        assert_eq!(process_transactions.locked, false);
    }

    #[test]
    fn handle_resolved_amount() {
        let mut test_obj = TransactionProcessor::new(1);
        let deposit_transaction_1 = TransactionInput {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(100.0),
        };
        let deposit_transaction_2 = TransactionInput {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 2,
            amount: Some(50.0),
        };
        let disputed_transaction = TransactionInput {
            transaction_type: TransactionType::Dispute,
            client: 1,
            tx: 2,
            amount: None,
        };
        let resolved_transaction = TransactionInput {
            transaction_type: TransactionType::Resolve,
            client: 1,
            tx: 2,
            amount: None,
        };

        test_obj.add_transaction(deposit_transaction_1);
        test_obj.add_transaction(deposit_transaction_2);
        test_obj.add_transaction(disputed_transaction);
        test_obj.add_transaction(resolved_transaction);

        let process_transactions = test_obj.process_transactions();
        assert_eq!(process_transactions.total, 150.0_f64);
        assert_eq!(process_transactions.held, 0.0_f64);
        assert_eq!(process_transactions.available, 150.0_f64);
        assert_eq!(process_transactions.locked, false);
    }

    #[test]
    fn handle_chargeback_deposit_amount() {
        let mut test_obj = TransactionProcessor::new(1);
        let deposit_transaction_1 = TransactionInput {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(100.0),
        };
        let deposit_transaction_2 = TransactionInput {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 2,
            amount: Some(50.0),
        };
        let disputed_transaction = TransactionInput {
            transaction_type: TransactionType::Dispute,
            client: 1,
            tx: 2,
            amount: None,
        };
        let chargeback_transaction = TransactionInput {
            transaction_type: TransactionType::Chargeback,
            client: 1,
            tx: 2,
            amount: None,
        };

        test_obj.add_transaction(deposit_transaction_1);
        test_obj.add_transaction(deposit_transaction_2);
        test_obj.add_transaction(disputed_transaction);
        test_obj.add_transaction(chargeback_transaction);
        let process_transactions = test_obj.process_transactions();
        assert_eq!(process_transactions.total, 100.0_f64);
        assert_eq!(process_transactions.held, 0.0_f64);
        assert_eq!(process_transactions.available, 100.0_f64);
        assert_eq!(process_transactions.locked, true);
    }
    #[test]
    fn handle_chargeback_withdrawal_amount() {
        let mut test_obj = TransactionProcessor::new(1);
        let deposit_transaction_1 = TransactionInput {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(100.0),
        };
        let deposit_transaction_2 = TransactionInput {
            transaction_type: TransactionType::Withdrawal,
            client: 1,
            tx: 2,
            amount: Some(50.0),
        };
        let disputed_transaction = TransactionInput {
            transaction_type: TransactionType::Dispute,
            client: 1,
            tx: 2,
            amount: None,
        };
        let chargeback_transaction = TransactionInput {
            transaction_type: TransactionType::Chargeback,
            client: 1,
            tx: 2,
            amount: None,
        };

        test_obj.add_transaction(deposit_transaction_1);
        test_obj.add_transaction(deposit_transaction_2);
        test_obj.add_transaction(disputed_transaction);
        test_obj.add_transaction(chargeback_transaction);
        let process_transactions = test_obj.process_transactions();
        assert_eq!(process_transactions.total, 100.0_f64);
        assert_eq!(process_transactions.held, 0.0_f64);
        assert_eq!(process_transactions.available, 100.0_f64);
        assert_eq!(process_transactions.locked, true);
    }

    #[test]
    fn handle_ignore_chargeback_if_invalid_transaction() {
        let mut test_obj = TransactionProcessor::new(1);
        let deposit_transaction_1 = TransactionInput {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(100.0),
        };
        let deposit_transaction_2 = TransactionInput {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 2,
            amount: Some(50.0),
        };
        let disputed_transaction = TransactionInput {
            transaction_type: TransactionType::Dispute,
            client: 1,
            tx: 2,
            amount: None,
        };
        let chargeback_transaction = TransactionInput {
            transaction_type: TransactionType::Chargeback,
            client: 1,
            tx: 99,
            amount: None,
        };

        test_obj.add_transaction(deposit_transaction_1);
        test_obj.add_transaction(deposit_transaction_2);
        test_obj.add_transaction(disputed_transaction);
        test_obj.add_transaction(chargeback_transaction);
        let process_transactions = test_obj.process_transactions();
        assert_eq!(process_transactions.total, 150.0_f64);
        assert_eq!(process_transactions.held, 50.0_f64);
        assert_eq!(process_transactions.available, 100.0_f64);
        assert_eq!(process_transactions.locked, false);
    }
    #[test]
    fn handle_ignore_dispute_if_invalid_transaction() {
        let mut test_obj = TransactionProcessor::new(1);
        let deposit_transaction_1 = TransactionInput {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(100.0),
        };
        let deposit_transaction_2 = TransactionInput {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 2,
            amount: Some(50.0),
        };
        let disputed_transaction = TransactionInput {
            transaction_type: TransactionType::Dispute,
            client: 1,
            tx: 99,
            amount: None,
        };

        test_obj.add_transaction(deposit_transaction_1);
        test_obj.add_transaction(deposit_transaction_2);
        test_obj.add_transaction(disputed_transaction);
        let process_transactions = test_obj.process_transactions();
        assert_eq!(process_transactions.total, 150.0_f64);
        assert_eq!(process_transactions.available, 150.0_f64);
        assert_eq!(process_transactions.locked, false);
    }

    #[test]
    fn handle_ignore_resolve_if_invalid_transaction() {
        let mut test_obj = TransactionProcessor::new(1);
        let deposit_transaction_1 = TransactionInput {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(100.0),
        };
        let deposit_transaction_2 = TransactionInput {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 2,
            amount: Some(50.0),
        };
        let disputed_transaction = TransactionInput {
            transaction_type: TransactionType::Dispute,
            client: 1,
            tx: 2,
            amount: None,
        };
        let resolved_transaction = TransactionInput {
            transaction_type: TransactionType::Dispute,
            client: 1,
            tx: 99,
            amount: None,
        };

        test_obj.add_transaction(deposit_transaction_1);
        test_obj.add_transaction(deposit_transaction_2);
        test_obj.add_transaction(disputed_transaction);
        test_obj.add_transaction(resolved_transaction);
        let process_transactions = test_obj.process_transactions();
        assert_eq!(process_transactions.total, 150.0_f64);
        assert_eq!(process_transactions.held, 50.0_f64);
        assert_eq!(process_transactions.available, 100.0_f64);
        assert_eq!(process_transactions.locked, false);
    }

    #[test]
    fn handle_ignore_deposit_if_zero_amount() {
        let mut test_obj = TransactionProcessor::new(1);
        let deposit_transaction_1 = TransactionInput {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(100.0),
        };
        let deposit_transaction_2 = TransactionInput {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 2,
            amount: Some(0.0),
        };

        test_obj.add_transaction(deposit_transaction_1);
        test_obj.add_transaction(deposit_transaction_2);
        let process_transactions = test_obj.process_transactions();
        assert_eq!(process_transactions.total, 100.0_f64);
        assert_eq!(process_transactions.held, 0.0_f64);
        assert_eq!(process_transactions.available, 100.0_f64);
        assert_eq!(process_transactions.locked, false);
    }
    #[test]

    fn handle_ignore_deposit_if_infinity_amount() {
        let mut test_obj = TransactionProcessor::new(1);
        let deposit_transaction_1 = TransactionInput {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(f64::MAX),
        };
        let deposit_transaction_2 = TransactionInput {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 2,
            amount: Some(1.0),
        };

        test_obj.add_transaction(deposit_transaction_1);
        test_obj.add_transaction(deposit_transaction_2);
        let process_transactions = test_obj.process_transactions();
        assert_eq!(process_transactions.total, f64::MAX);
        assert_eq!(process_transactions.held, 0.0_f64);
        assert_eq!(process_transactions.available, f64::MAX);
        assert_eq!(process_transactions.locked, false);
    }
}
