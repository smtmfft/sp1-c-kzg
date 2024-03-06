//! A simple script to generate and verify the proof of a given program.

use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

// fn read_kzg_setup() -> Vec<u8> {
//     let b_file_exists = std::path::Path::new("./kzg_parsed_trust_setup").exists();
//     assert!(b_file_exists);
//     // open file as lines of strings
//     let kzg_trust_setup_str = std::fs::read_to_string("../kzg_parsed_trust_setup").unwrap();
//     let (g1, g2) = parse_kzg_trusted_setup(&kzg_trust_setup_str)
//         .map_err(|e| {
//             println!("error: {:?}", e);
//             e
//         })
//         .unwrap();
//     let kzg_settings = KzgSettings::load_trusted_setup(&g1.0, &g2.0).unwrap();
//     let kzg_commit = KzgCommitment::blob_to_kzg_commitment(&blob, &kzg_settings).unwrap();
//     assert_eq!(
//         kzg_to_versioned_hash(kzg_commit).to_string(),
//         "0x010657f37554c781402a22917dee2f75def7ab966d7b770905398eba3c444014"
//     );
// }

fn main() {
    // Generate proof.
    let mut stdin = SP1Stdin::new();
    // let n = 186u32;
    let blob: Vec<u8> = [0u8; 131072].into();
    stdin.write(&blob);
    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    // Read output.
    let commit = proof.stdout.read::<u128>();
    println!("commit: {}", commit);

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("succesfully generated and verified proof for the program!")
}
