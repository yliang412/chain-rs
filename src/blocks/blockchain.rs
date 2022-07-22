use crate::Block;
use tracing::info;

/// Hard-coded constant of required leading zeros needed in hash
const CURR_BITS: usize = 8;

/// A structure representing the blockchain
pub struct Blockchain {
    /// A list of blocks in the blockchain
    blocks: Vec<Block>,
    /// Number of useful blocks in the blockchain (not including genesis block)
    height: usize,
}

impl Blockchain {
    /// Creates a new blockchain with only a genesis block
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![Block::create_genesis_block(CURR_BITS)],
            height: 0,
        }
    }

    /// Adds a new block with given block data to the blockchain
    pub fn mine_block(&mut self, data: &str) {
        let prev_block = self.blocks.last().unwrap();
        let block = Block::new(data, prev_block.get_hash().as_str(), CURR_BITS);
        self.blocks.push(block);
        self.height += 1
    }

    /// Logs block information
    pub fn blocks_info(&self) {
        for block in self.blocks.iter() {
            info!("{:#?}", block);
        }
    }
}
