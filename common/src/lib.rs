use bitcoin_hashes::{sha256d, Hash};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

fn genesis_root() -> [u8; 32] {
    hex::decode("3ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a")
        .unwrap()
        .try_into()
        .unwrap()
}

static GENESIS_MERKLE_ROOT: Lazy<[u8; 32]> = Lazy::new(genesis_root);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Header {
    pub version: i32,
    pub prev_blockhash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub time: u32,
    pub bits: u32,
    pub nonce: u32,
}

// Example of how to create a sample block header
pub fn create_genesis_block_header() -> Header {
    Header {
        version: 0x01,
        prev_blockhash: [0u8; 32],
        merkle_root: *GENESIS_MERKLE_ROOT,
        time: 0x495fab29,
        bits: 0x1d00ffff,
        nonce: 0x7c2bac1d,
    }
}

// Function to serialize a block header to bytes
pub fn serialize_block_header(header: &Header) -> Vec<u8> {
    let mut result = vec![];
    result.extend_from_slice(&header.version.to_le_bytes());
    result.extend_from_slice(&header.prev_blockhash);
    result.extend_from_slice(&header.merkle_root);
    result.extend_from_slice(&header.time.to_le_bytes());
    result.extend_from_slice(&header.bits.to_le_bytes());
    result.extend_from_slice(&header.nonce.to_le_bytes());
    result
}

// Function to calculate the double SHA-256 hash of a block header
pub fn calculate_hash(header: &Header) -> [u8; 32] {
    let serialized = serialize_block_header(header);
    println!("serialized: {:?}", hex::encode(&serialized));
    *sha256d::Hash::hash(&serialized).as_byte_array()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_genesis() {
        let header = crate::create_genesis_block_header();
        let mut hash = super::calculate_hash(&header);
        hash.reverse();
        let expected: [u8; 32] =
            hex::decode("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f")
                .unwrap()
                .try_into()
                .unwrap();
        assert_eq!(hash, expected);
    }
}

// #[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
// pub struct Transaction {
//     /// The protocol version, is currently expected to be 1 or 2 (BIP 68).
//     pub version: i32,
//     /// Block height or timestamp. Transaction cannot be included in a block until this height/time.
//     ///
//     /// ### Relevant BIPs
//     ///
//     /// * [BIP-65 OP_CHECKLOCKTIMEVERIFY](https://github.com/bitcoin/bips/blob/master/bip-0065.mediawiki)
//     /// * [BIP-113 Median time-past as endpoint for lock-time calculations](https://github.com/bitcoin/bips/blob/master/bip-0113.mediawiki)
//     pub lock_time: LockTime,
//     /// List of transaction inputs.
//     pub input: Vec<TxIn>,
//     /// List of transaction outputs.
//     pub output: Vec<TxOut>,
// }
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
// pub struct Time(u32);
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
// pub struct Height(u32);
//
// #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
// pub enum LockTime {
//     Blocks(Height),
//     Seconds(Time),
// }
//
// #[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
// pub struct Block {
//     /// The block header
//     pub header: Header,
//     /// List of transactions contained in the block
//     pub txdata: Vec<Transaction>,
// }
//
