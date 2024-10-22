#![no_main]
use common::Header;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    let header: Header = env::read();

    let mut hash = header.calculate_hash();
    hash.reverse();
    let expected: [u8; 32] =
        hex::decode("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f")
            .unwrap()
            .try_into()
            .unwrap();
    assert_eq!(hash, expected);
    assert_eq!(header.version, 1);
    assert_eq!(header.time, 0x495fab29);
    assert!(header.bits >= 0x1d00ffff);
    assert_eq!(header.nonce, 0x7c2bac1d);
    assert!(header.validate_target(header.target_from_bits()));

    // write public output to the journal
    env::commit(&header);
}
