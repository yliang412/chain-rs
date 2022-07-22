use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::ProofOfWork;

/// A block header storing metadata
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct BlockHeader {
    /// Timestamp at creation of the block
    timestamp: i64,
    /// Hash of the previous block
    prev_hash: String,
    /// Number of leading zero bits (PoW hardness)
    bits: usize,
    /// Number of computation needed to satisfy the hardness
    nonce: usize,
}

impl BlockHeader {
    fn new(prev_hash: &str, bits: usize) -> Self {
        BlockHeader {
            timestamp: Utc::now().timestamp(),
            prev_hash: prev_hash.into(),
            bits,
            nonce: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Block {
    /// A block header storing metadata
    header: BlockHeader,
    /// Data of the block
    data: String,
    /// Hash of the current block
    hash: String,
}

impl Block {
    /// Returns a new block with given data and previous hash value
    ///
    /// # Arguments
    ///
    /// * `data` - A string slice that holds the data of the block
    /// * `prev_hash` - A string slice that holds the hash of previous block
    pub fn new(data: &str, prev_hash: &str, bits: usize) -> Self {
        let mut block = Block {
            header: BlockHeader::new(prev_hash, bits),
            data: data.into(),
            hash: String::new(),
        };
        let pow = ProofOfWork::new(bits);
        pow.run(&mut block);

        block
    }

    /// Returns the genesis block
    pub fn create_genesis_block(bits: usize) -> Self {
        Self::new("GenesisBlock", "", bits)
    }

    /// Gets the hash of the current block
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    /// Sets the hash of the current block based on the header
    pub fn set_hash(&mut self, hash: String) {
        self.hash = hash;
    }

    pub fn get_header(&self) -> BlockHeader {
        self.header.clone()
    }

    pub fn set_nonce(&mut self, nonce: usize) {
        self.header.nonce = nonce;
    }
}
