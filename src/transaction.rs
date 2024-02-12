// transaction.rs
use std::collections::HashSet;

#[derive(Debug)]
pub struct Transaction {
    pub txid: String,
    pub fee: u32,
    pub weight: u32,
    pub parent_txids: HashSet<String>,
}
