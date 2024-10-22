use common::create_genesis_block_header;
use common::Header;
use methods::{VERIFY_ELF, VERIFY_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let input: Header = create_genesis_block_header();

    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, VERIFY_ELF).unwrap().receipt;

    let output: Header = receipt.journal.decode().unwrap();
    println!("time: {}", output.time);

    // The receipt was verified at the end of proving, but the below code is an
    // example of how someone else could verify this receipt.
    receipt.verify(VERIFY_ID).unwrap();
}
