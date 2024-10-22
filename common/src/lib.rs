use std::ops::Shl;

use bitcoin_hashes::{sha256d, Hash};
use serde::{Deserialize, Serialize};

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
    let merkle_root =
        hex::decode("3ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a")
            .unwrap()
            .try_into()
            .unwrap();

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
    pub fn serialize_block_header(&self) -> Vec<u8> {
        let mut result = vec![];
        result.extend_from_slice(&self.version.to_le_bytes());
        result.extend_from_slice(&self.prev_blockhash);
        result.extend_from_slice(&self.merkle_root);
        result.extend_from_slice(&self.time.to_le_bytes());
        result.extend_from_slice(&self.bits.to_le_bytes());
        result.extend_from_slice(&self.nonce.to_le_bytes());
        result
    }

    // Function to calculate the double SHA-256 hash of a block header
    pub fn calculate_hash(&self) -> [u8; 32] {
        let serialized = self.serialize_block_header();
        sha256d::Hash::hash(&serialized).to_byte_array()
    }
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

// Function to serialize a block header to bytes
#[cfg(test)]
mod tests {

    #[test]
    fn test_genesis() {
        let header = crate::create_genesis_block_header();
        let mut hash = header.calculate_hash();
        hash.reverse();
        let expected: [u8; 32] =
            hex::decode("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f")
                .unwrap()
                .try_into()
                .unwrap();
        assert_eq!(hash, expected);
    }

    #[test]
    fn test_target() {
        let header = crate::create_genesis_block_header();
        // Validate the block's proof of work
        assert!(header.validate_target());
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
