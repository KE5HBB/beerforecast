use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::blockchain::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Master {
    pub master_blocks: Vec<MasterBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterBlock {
    pub id: i64,
    pub previous_hash: String,
    pub block_hash: String,
    pub block_data: Vec<Block>,
}

impl Master {
    pub fn new() -> Self {
        Self {
            master_blocks: vec![MasterBlock::new(
                0,
                "genesus".to_string(),
                [Block {
                    id: -1,
                    previous_hash: "abcd".to_string(),
                    block_hash: "efgh".to_string(),
                    transaction: Transaction {
                        sender: "test".to_string(),
                        reciever: "net".to_string(),
                        amount: 0.0,
                    },
                }]
                .to_vec(),
            )],
        }
   