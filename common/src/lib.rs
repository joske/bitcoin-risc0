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
        *sha256d::Hash::hash(&serialized).as_byte_array()
    }

    /// Convert compact bits format to the target 256-bit value.
    pub fn target_from_bits(&self) -> [u8; 32] {
        let bits = self.bits;
        let exponent = bits >> 24; // First byte (exponent)
        let mantissa = bits & 0x00ffffff; // Last 3 bytes (mantissa)

        // The target is calculated as: mantissa * 2^(8 * (exponent - 3))
        let mut target = [0u8; 32];

        if exponent <= 3 {
            // If the exponent is less than or equal to 3, the mantissa is shifted to the right.
            target[0..(exponent as usize)].copy_from_slice(&mantissa.to_be_bytes()[1..]);
        } else {
            // Shift mantissa left by (exponent - 3) bytes.
            let shift = (exponent - 3) as usize;
            target[shift..shift + 3].copy_from_slice(&mantissa.to_be_bytes()[1..]);
        }

        target
    }

    /// Validate the proof of work by checking if the block hash is less than or equal to the target.
    pub fn validate_target(&self, required_target: [u8; 32]) -> bool {
        let block_hash = self.calculate_hash();
        let target = self.target_from_bits();
        println!("required_target: {:x?}", required_target);
        println!("Target:          {:x?}", target);
        println!("Block hash:      {:x?}", block_hash);
        if target != required_target {
            return false;
        }

        // Compare the block hash with the target using lexicographical comparison
        block_hash <= target
    }
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
        let mut hash = header.calculate_hash();
        hash.reverse();

        // Validate the block's proof of work
        assert!(header.validate_target(header.target_from_bits()));
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
