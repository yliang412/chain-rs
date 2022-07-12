use crypto::{digest::Digest, sha3::Sha3};
use serde::Serialize;

use crate::error::BlockchainError;

type Result<T> = anyhow::Result<T, BlockchainError>;

pub fn serialize<T>(data: &T) -> Result<Vec<u8>>
where
    T: Serialize + ?Sized,
{
    Ok(bincode::serialize(data)?)
}

pub fn hash_to_str(data: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(data);
    hasher.result_str()
}
