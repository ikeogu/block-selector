// main.rs

mod transaction;
mod block;

use std::collections::HashMap;
use crate::transaction::Transaction;
use crate::block::construct_block;
use std::collections::HashSet;
use std::io;
use std::fs;



fn parse_transaction(line: &str) -> Option<Transaction> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() < 3 {
        return None;
    }
    let txid = parts[0].to_string();
    let fee = parts[1].parse().ok()?;
    let weight = parts[2].parse().ok()?;
    let parent_txids: HashSet<String> = if parts.len() > 3 {
        parts[3].split(';').map(|s| s.to_string()).collect()
    } else {
        HashSet::new()
    };
    Some(Transaction {
        txid,
        fee,
        weight,
        parent_txids,
    })
}

 fn read_mempool(filename: &str) -> io::Result<HashMap<String, Transaction>> {
    let mut mempool = HashMap::new();
    let content = fs::read_to_string(filename)?;

    for line in content.lines() {
        if let Some(transaction) = parse_transaction(line) {
            mempool.insert(transaction.txid.clone(), transaction);
        } else {
            eprintln!("Failed to parse transaction: {}", line);
        }
    }

    Ok(mempool)
} 

    
fn main() {
    let mempool_result = read_mempool("data/mempool.csv");
    let mempool = match mempool_result {
        
        Ok(mempool) => mempool,
        Err(err) => {
            eprintln!("Error reading mempool: {}", err);
            return;
        }
    };
    
    

    let max_block_weight = 4000000; // Example value, adjust as needed
    let block = construct_block(&mempool, max_block_weight, &HashSet::new());
    
    for txid in block {
        println!("{}", txid);
    }
}
