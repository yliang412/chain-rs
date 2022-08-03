use crate::{error::BlockchainError, Block};

mod sleddb;

pub use sleddb::SledDb;

/// Hash of the last added block
pub const TIP_KEY: &str = "tip_hash";

/// Height of the blockchain
pub const HEIGHT: &str = "height";

/// Table name of the blocks store
pub const TABLE_OF_BLOCKS: &str = "blocks";

pub trait Storage: Send + Sync + 'static {
    /// Get hash value of the last block
    fn get_tip(&self) -> Result<Option<String>, BlockchainError>;

    /// Get block associated with hash key
    fn get_block(&self, key: &str) -> Result<Option<Block>, BlockchainError>;

    /// Get the blockchain height
    fn get_height(&self) -> Result<Option<usize>, BlockchainError>;

    /// Update blockchain in a transaction
    fn update_blocks(&self, key: &str, block: &Block, height: usize);

    /// Get block iterator
    fn get_block_iter(&self) -> Result<Box<dyn Iterator<Item = Block>>, BlockchainError>;
}

pub struct StorageIterator<T> {
    data: T,
}

impl<T> StorageIterator<T> {
    pub fn new(data: T) -> Self {
        StorageIterator { data }
    }
}

/// An iterator accessing block storage
impl<T> Iterator for StorageIterator<T>
where
    T: Iterator,
    T::Item: Into<Block>,
{
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().map(|v| v.into())
    }
}
