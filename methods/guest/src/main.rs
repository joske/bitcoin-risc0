#![no_main]
use common::Block;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    let block: Block = env::read();
    let header = block.header;

    let mut hash = block.calculate_block_hash();
    hash.reverse();
    let expected: [u8; 32] =
        hex::decode("00000000839a8e6886ab5951d76f411475428afc90947ee320161bbf18eb6048")
            .unwrap()
            .try_into()
            .unwrap();
    assert_eq!(hash, expected);
    assert_eq!(header.version, 1);
    assert_eq!(header.time, 0x495fab29);
    let mut merkle_root = block.calculate_merkle_root();
    merkle_root.reverse();
    assert_eq!(header.merkle_root, merkle_root);
    assert!(header.bits >= 0x1d00ffff);
    assert_eq!(header.nonce, 0x7c2bac1d);
    assert!(header.validate_target());

    // write public output to the journal
    env::commit(&header);
}
