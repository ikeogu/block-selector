use crate::transaction::Transaction;
use std::collections::{HashMap, HashSet};

pub fn construct_block(
    mempool: &HashMap<String, Transaction>,
    max_block_weight: u32,
    _: &HashSet<String>, // Unused HashSet parameter
) -> Vec<String> {
    let mut block = Vec::new();
    let mut included = HashSet::new();
    let mut remaining_capacity = max_block_weight as i32;
    let mut total_fee = 0;

    // Sort transactions by fee-to-weight ratio in descending order
    let mut sorted_txids: Vec<_> = mempool.keys().collect();
    sorted_txids.sort_by_key(|&txid| {
        let transaction = &mempool[txid];
        (transaction.fee as f64 / transaction.weight as f64 * 1000.0) as u32
    });
    sorted_txids.reverse();

    // Greedy approach: iteratively add transactions to the block
    for &txid in &sorted_txids {
        let transaction = &mempool[txid];
        
        // Check if the transaction has not been included before
        if !included.contains(txid) {
            // Check if all parent transactions are already included
           if transaction.parent_txids.iter().all(|pid| included.contains(pid)) {
                if transaction.weight as i32 <= remaining_capacity {
                    block.push(txid.clone());
                    included.insert(txid.clone());
                    remaining_capacity -= transaction.weight as i32;
                    total_fee += transaction.fee;
                }
            }
        }
    }

    println!("Total fee earned: {}", total_fee);
    block
}
