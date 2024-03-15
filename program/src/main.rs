//! A simple program to be proven inside the zkVM.

#![no_main]

sp1_zkvm::entrypoint!(main);

use kzg_utils::kzg_trust_setup::KZG_TRUST_SETUP;
use kzg_utils::kzg_utils::{kzg_to_versioned_hash, parse_kzg_trusted_setup};
use kzg_utils::{KzgCommitment, KzgSettings};

const KZG_DATA: &[u8] = include_bytes!("../../kzg_settings_raw.bin");

pub fn main() {
    // NOTE: values of n larger than 186 will overflow the u128 type,
    // resulting in output that doesn't match fibonacci sequence.
    // However, the resulting proof will still be valid!
    let _n: Vec<u8> = sp1_zkvm::io::read::<Vec<u8>>();
    let mut data = Vec::from(KZG_DATA);
    let kzg_settings = KzgSettings::from_u8_slice(&mut data);
    let blob = [0; 4096 * 32].into();
    let kzg_commit = KzgCommitment::blob_to_kzg_commitment(&blob, &kzg_settings).unwrap();
    let versioned_hash = kzg_to_versioned_hash(&kzg_commit);
    sp1_zkvm::io::write(&versioned_hash);
}
