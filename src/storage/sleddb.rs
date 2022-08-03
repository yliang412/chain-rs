use sled::{transaction::TransactionResult, Db, IVec};
use std::path::Path;

use crate::{
    utils::{deserialize, serialize},
    Block, Storage, StorageIterator, HEIGHT, TABLE_OF_BLOCKS, TIP_KEY,
};

pub struct SledDb {
    /// Sled db instance
    db: Db,
}

impl SledDb {
    /// Creates a new SledDb instance
    pub fn new(path: impl AsRef<Path>) -> Self {
        SledDb {
            db: sled::open(path).unwrap(),
        }
    }

    /// Gets the full key of the table
    fn get_full_key(table: &str, key: &str) -> String {
        format!("{}:{}", table, key)
    }
}

impl Storage for SledDb {
    fn get_tip(&self) -> Result<Option<String>, crate::error::BlockchainError> {
        let res = self
            .db
            .get(TIP_KEY)?
            .map(|v| deserialize::<String>(&v.to_vec()));
        res.map_or(Ok(None), |v| v.map(Some))
    }

    fn get_block(&self, key: &str) -> Result<Option<crate::Block>, crate::error::BlockchainError> {
        let name = Self::get_full_key(TABLE_OF_BLOCKS, key);
        let res = self.db.get(name)?.map(|v| v.into());
        Ok(res)
    }

    fn get_height(&self) -> Result<Option<usize>, crate::error::BlockchainError> {
        let res = self
            .db
            .get(HEIGHT)?
            .map(|v| deserialize::<usize>(&v.to_vec()));
        res.map_or(Ok(None), |v| v.map(Some))
    }

    fn update_blocks(&self, key: &str, block: &crate::Block, height: usize) {
        let _: TransactionResult<(), ()> = self.db.transaction(|db| {
            let name = Self::get_full_key(TABLE_OF_BLOCKS, key);
            db.insert(name.as_str(), serialize(block).unwrap())?;
            db.insert(TIP_KEY, serialize(key).unwrap())?;
            db.insert(HEIGHT, serialize(&height).unwrap())?;
            db.flush();
            Ok(())
        });
    }

    fn get_block_iter(
        &self,
    ) -> Result<Box<dyn Iterator<Item = Block>>, crate::error::BlockchainError> {
        let prefix = format!("{}:", TABLE_OF_BLOCKS);
        let iter = StorageIterator::new(self.db.scan_prefix(prefix));
        Ok(Box::new(iter))
    }
}

impl From<IVec> for Block {
    fn from(v: IVec) -> Self {
        let result = deserialize::<Block>(&v.to_vec());
        match result {
            Ok(block) => block,
            Err(_) => Block::default(),
        }
    }
}

impl From<Result<(IVec, IVec), sled::Error>> for Block {
    fn from(res: Result<(IVec, IVec), sled::Error>) -> Self {
        match res {
            Ok((_, v)) => match deserialize::<Block>(&v.to_vec()) {
                Ok(block) => block,
                Err(_) => Block::default(),
            },
            Err(_) => Block::default(),
        }
    }
}
