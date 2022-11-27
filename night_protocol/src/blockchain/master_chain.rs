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
    }

    pub fn add_master_block(&mut self, block_data: Vec<Block>) {
        if self.validate_chain() == true {
            let prev_block = &self.master_blocks[&self.master_blocks.len() - 1];
            let new_block =
                MasterBlock::new(prev_block.id + 1, prev_block.clone().block_hash, block_data);
            self.master_blocks.push(new_block);
        } else {
            return
        }
    }

    pub fn validate_master_block(&self, block: &MasterBlock, previous_block: &MasterBlock) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}-{}-{:?}", previous_block.id, previous_block.previous_hash, previous_block.block_data));
        let previous_block_hash: String = format!("{:x}", hasher.finalize());

        if block.previous_hash.trim() == previous_block_hash.trim() {
            true
        } else {
            false
        }
    }

    pub fn v