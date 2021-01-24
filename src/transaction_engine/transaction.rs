use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Deserialize)]
pub struct TransactionInput {
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,

    #[serde(rename = "client")]
    pub client: u16,

    #[serde(rename = "tx")]
    pub tx: u32,

    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "amount")]
    pub amount: Option<f64>,
}
