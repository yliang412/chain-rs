use anyhow::Result;
use crypto::{digest::Digest, sha3::Sha3};
use serde::{Deserialize, Serialize};

use crate::error::BlockchainError;

/// Serializes data of type T to a vector of bytes
pub fn serialize<T>(data: &T) -> Result<Vec<u8>, BlockchainError>
where
    T: Serialize + ?Sized,
{
    Ok(bincode::serialize(data)?)
}

/// Deserializes from a slice of bytes to data of type T
#[allow(dead_code)]
pub fn deserialze<'a, T>(data: &'a [u8]) -> Result<T, BlockchainError>
where
    T: Deserialize<'a> + ?Sized,
{
    Ok(bincode::deserialize(data)?)
}

/// Converts hash value to string
pub fn hash_to_str(data: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(data);
    hasher.result_str()
}

/// Converts hash value to bytes
pub fn hash_to_u8(data: &[u8], out: &mut [u8]) {
    let mut hasher = Sha3::sha3_256();
    hasher.input(data);
    hasher.result(out)
}
