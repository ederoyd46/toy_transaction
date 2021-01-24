use serde::{Serialize, Serializer};

#[derive(Debug, Serialize)]
pub struct TransactionRunningState {
    pub client: u16,
    #[serde(serialize_with = "round_serialize")]
    pub available: f64,
    #[serde(serialize_with = "round_serialize")]
    pub held: f64,
    #[serde(serialize_with = "round_serialize")]
    pub total: f64,
    pub locked: bool,
}

fn round_serialize<S: Serializer>(value: &f64, s: S) -> Result<S::Ok, S::Error> {
    // Round to a maximum of 4 decimal places
    s.serialize_f64((value * 10000.0).round() / 10000.0)
}

impl TransactionRunningState {
    pub fn new(client: u16) -> Self {
        Self {
            client,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        let new_total = self.total + amount;

        if new_total > self.total {
            self.total = new_total;
            self.calculate_available();
        } else {
            eprintln!("Zero or Infinite amount detected, ignoring deposit");
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        if self.available >= amount {
            self.total -= amount;
            self.calculate_available();
        } else {
            eprintln!("Withdrawal amount ignored as there is not enough available balance");
        }
    }

    // TODO Refactor Chargeback should be one function
    pub fn chargeback_deposit(&mut self, amount: f64) {
        self.total -= amount;
        self.chargeback(amount);
    }

    pub fn chargeback_withdrawal(&mut self, amount: f64) {
        self.total += amount;
        self.chargeback(amount);
    }

    pub fn hold(&mut self, amount: f64) {
        self.held += amount;
        self.calculate_available();
    }

    pub fn release(&mut self, amount: f64) {
        self.held -= amount;
        self.calculate_available();
    }

    fn chargeback(&mut self, amount: f64) {
        self.held -= amount;
        self.locked = true;
        self.calculate_available();
    }

    fn calculate_available(&mut self) {
        self.available = self.total - self.held
    }
}
