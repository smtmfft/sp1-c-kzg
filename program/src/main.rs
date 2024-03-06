//! A simple program to be proven inside the zkVM.

#![no_main]

sp1_zkvm::entrypoint!(main);

use kzg_utils::kzg_trust_setup::KZG_TRUST_SETUP;
use kzg_utils::kzg_utils::{kzg_to_versioned_hash, parse_kzg_trusted_setup};
use kzg_utils::{KzgCommitment, KzgSettings};

pub fn main() {
    // NOTE: values of n larger than 186 will overflow the u128 type,
    // resulting in output that doesn't match fibonacci sequence.
    // However, the resulting proof will still be valid!
    let _n: Vec<u8> = sp1_zkvm::io::read::<Vec<u8>>();
    let (g1, g2) = parse_kzg_trusted_setup(&KZG_TRUST_SETUP)
        .map_err(|e| {
            println!("error: {:?}", e);
            e
        })
        .unwrap();
    let kzg_settings = KzgSettings::load_trusted_setup(&g1.0, &g2.0).unwrap();
    let blob = [0; 4096 * 32].into();
    let kzg_commit = KzgCommitment::blob_to_kzg_commitment(&blob, &kzg_settings).unwrap();
    let versioned_hash = kzg_to_versioned_hash(&kzg_commit);
    sp1_zkvm::io::write(&versioned_hash);
}
