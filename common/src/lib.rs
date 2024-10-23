use std::ops::Shl;

use bitcoin_hashes::{sha256d, Hash};
use serde::{Deserialize, Serialize};

macro_rules! from_hex {
    ($s:literal) => {
        hex::decode($s).unwrap().try_into().unwrap()
    };
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub struct Transaction {
    data: Vec<u8>,
}

impl Transaction {
    pub fn txid(&self) -> [u8; 32] {
        let mut serialized = vec![];
        serialized.extend_from_slice(&self.data);
        println!("serialized tx: {:x?}", serialized);
        sha256d::Hash::hash(&serialized).to_byte_array()
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub struct Block {
    /// The block header
    pub header: Header,
    /// List of transactions contained in the block
    pub txdata: Vec<Transaction>,
}

impl Block {
    /// Calculate the merkle root of the block
    pub fn calculate_merkle_root(&self) -> [u8; 32] {
        let hashes: Vec<[u8; 32]> = self.txdata.iter().map(|tx| tx.txid()).collect();
        calculate_merkle_root(hashes)
    }

    pub fn calculate_block_hash(&self) -> [u8; 32] {
        self.header.calculate_hash()
    }
}

/// Calculate the Merkle root from a list of transaction hashes.
fn calculate_merkle_root(mut hashes: Vec<[u8; 32]>) -> [u8; 32] {
    // Return the only hash if we have a single transaction (special case)
    if hashes.len() == 1 {
        return hashes[0];
    }

    // Keep hashing pairs of hashes until we reach a single hash (the root)
    while hashes.len() > 1 {
        let mut new_level = Vec::new();

        for i in (0..hashes.len()).step_by(2) {
            if i + 1 < hashes.len() {
                // Hash the pair of hashes
                let mut combined = Vec::new();
                combined.extend_from_slice(&hashes[i]);
                combined.extend_from_slice(&hashes[i + 1]);
                new_level.push(sha256d::Hash::hash(&combined).to_byte_array());
            } else {
                // If we have an odd number of hashes, duplicate the last one
                let mut combined = Vec::new();
                combined.extend_from_slice(&hashes[i]);
                combined.extend_from_slice(&hashes[i]);
                new_level.push(sha256d::Hash::hash(&combined).to_byte_array());
            }
        }

        hashes = new_level;
    }

    // The last remaining hash is the Merkle root
    hashes[0]
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Header {
    pub version: i32,
    pub prev_blockhash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub time: u32,
    pub bits: u32,
    pub nonce: u32,
}

pub fn create_genesis_block() -> Block {
    let header = create_genesis_block_header();
    let tx = hex::decode("0101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff4d04ffff001d0104455468652054696d65732030332f4a616e2f32303039204368616e63656c6c6f72206f6e206272696e6b206f66207365636f6e64206261696c6f757420666f722062616e6b73ffffffff0100f2052a01000000434104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac00000000").unwrap();
    let txdata = vec![Transaction { data: tx }];
    Block { header, txdata }
}

pub fn create_block_1() -> Block {
    let header = Header {
        version: 0x01,
        prev_blockhash: from_hex!(
            "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"
        ),
        merkle_root: from_hex!("0e3e2357e806b6cdb1f70b54c3a3a17b6714ee1f0e68bebb44a74b1efd512098"),
        time: 0x495fab29,
        bits: 0x1d00ffff,
        nonce: 0x7c2bac1d,
    };
    let data = from_hex!("01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0704ffff001d0104ffffffff0100f2052a0100000043410496b538e853519c726a2c91e61ec11600ae1390813a627c66fb8be7947be63c52da7589379515d4e0a604f8141781e62294721166bf621e73a82cbf2342c858eeac00000000");
    let txdata = vec![Transaction { data }];

    Block { header, txdata }
}

/// Create the genesis block header for the Bitcoin blockchain.
pub fn create_genesis_block_header() -> Header {
    let merkle_root = from_hex!("3ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a");

    Header {
        version: 0x01,
        prev_blockhash: [0u8; 32],
        merkle_root,
        time: 0x495fab29,
        bits: 0x1d00ffff,
        nonce: 0x7c2bac1d,
    }
}

impl Header {
    fn serialize_block_header(&self) -> Vec<u8> {
        let mut result = vec![];
        result.extend_from_slice(&self.version.to_le_bytes());
        result.extend_from_slice(&self.prev_blockhash);
        result.extend_from_slice(&self.merkle_root);
        result.extend_from_slice(&self.time.to_le_bytes());
        result.extend_from_slice(&self.bits.to_le_bytes());
        result.extend_from_slice(&self.nonce.to_le_bytes());
        result
    }

    /// calculate the double SHA-256 hash of a block header
    pub fn calculate_hash(&self) -> [u8; 32] {
        let serialized = self.serialize_block_header();
        sha256d::Hash::hash(&serialized).to_byte_array()
    }

    /// Extract the target from the bits field of the block header
    fn target(&self) -> U256 {
        let bits = self.bits;
        // This is a floating-point "compact" encoding originally used by
        // OpenSSL, which satoshi put into consensus code, so we're stuck
        // with it. The exponent needs to have 3 subtracted from it, hence
        // this goofy decoding code. 3 is due to 3 bytes in the mantissa.
        let (mant, expt) = {
            let unshifted_expt = bits >> 24;
            if unshifted_expt <= 3 {
                ((bits & 0xFFFFFF) >> (8 * (3 - unshifted_expt as usize)), 0)
            } else {
                (bits & 0xFFFFFF, 8 * ((bits >> 24) - 3))
            }
        };

        // The mantissa is signed but may not be negative.
        if mant > 0x7F_FFFF {
            U256::ZERO
        } else {
            U256::from(mant) << expt
        }
    }

    /// Validate the proof of work by checking if the block hash is less than or equal to the target.
    pub fn validate_target(&self) -> bool {
        let block_hash = self.calculate_hash();
        let target = self.target();
        // println!("required_target: {:x?}", required_target);
        // println!("Target:          {:x?}", target);
        // println!("Block hash:      {:x?}", block_hash);

        // if target != required_target {
        //     return false;
        // }
        //
        let hash = U256::from_le_bytes(block_hash);
        // Compare the block hash with the target using lexicographical comparison
        hash <= target
    }
}

/// Big-endian 256 bit integer type.
// (high, low): u.0 contains the high bits, u.1 contains the low bits.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct U256(u128, u128);

impl U256 {
    const ZERO: U256 = U256(0, 0);

    /// Creates a `U256` from a little-endian array of `u8`s.
    fn from_le_bytes(a: [u8; 32]) -> U256 {
        let (high, low) = split_in_half(a);
        let little = u128::from_le_bytes(high);
        let big = u128::from_le_bytes(low);
        U256(big, little)
    }

    fn wrapping_shl(self, rhs: u32) -> Self {
        let shift = rhs & 0x000000ff;

        let mut ret = U256::ZERO;
        let word_shift = shift >= 128;
        let bit_shift = shift % 128;

        if word_shift {
            ret.0 = self.1 << bit_shift
        } else {
            ret.0 = self.0 << bit_shift;
            if bit_shift > 0 {
                ret.0 += self.1.wrapping_shr(128 - bit_shift);
            }
            ret.1 = self.1 << bit_shift;
        }
        ret
    }
}

impl Shl<u32> for U256 {
    type Output = Self;
    fn shl(self, shift: u32) -> U256 {
        self.wrapping_shl(shift)
    }
}

impl<T: Into<u128>> From<T> for U256 {
    fn from(x: T) -> Self {
        U256(0, x.into())
    }
}

/// Splits a 32 byte array into two 16 byte arrays.
fn split_in_half(a: [u8; 32]) -> ([u8; 16], [u8; 16]) {
    let mut high = [0_u8; 16];
    let mut low = [0_u8; 16];

    high.copy_from_slice(&a[..16]);
    low.copy_from_slice(&a[16..]);

    (high, low)
}

#[cfg(test)]
mod tests {
    use crate::{calculate_merkle_root, Block, Header, Transaction};

    #[test]
    fn test_merkle_root() {
        // block 1
        let hashes: Vec<[u8; 32]> = vec![from_hex!(
            "0e3e2357e806b6cdb1f70b54c3a3a17b6714ee1f0e68bebb44a74b1efd512098"
        )];
        let merkle_root = calculate_merkle_root(hashes);
        let expected: [u8; 32] =
            from_hex!("0e3e2357e806b6cdb1f70b54c3a3a17b6714ee1f0e68bebb44a74b1efd512098");
        assert_eq!(expected, merkle_root);
    }

    #[test]
    fn test_txid() {
        let block = crate::create_block_1();
        let mut txid = block.txdata[0].txid();
        txid.reverse();
        println!("tx id :{:x?}", txid);
        let expected: [u8; 32] =
            from_hex!("0e3e2357e806b6cdb1f70b54c3a3a17b6714ee1f0e68bebb44a74b1efd512098");

        assert_eq!(expected, txid);
        let mut merkle_root = block.calculate_merkle_root();
        let expected_merkle_root = block.header.merkle_root;
        merkle_root.reverse();
        assert_eq!(merkle_root, expected_merkle_root);
    }

    #[test]
    fn test_genesis() {
        let header = crate::create_genesis_block_header();
        let mut hash = header.calculate_hash();
        hash.reverse();
        let expected: [u8; 32] =
            from_hex!("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f");
        assert_eq!(hash, expected);
    }

    #[test]
    fn test_target() {
        let header = crate::create_genesis_block_header();
        assert!(header.validate_target());
    }
}
