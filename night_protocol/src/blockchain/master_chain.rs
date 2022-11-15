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
  