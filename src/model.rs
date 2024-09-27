use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub user_id: u32,
    pub amount: f64,
    pub r#type: TxnType,
    pub time: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TxnType {
    #[allow(dead_code)]
    #[serde(rename = "deposit")]
    Deposit,
    #[allow(dead_code)]
    #[serde(rename = "withdraw")]
    Withdrawal,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}
