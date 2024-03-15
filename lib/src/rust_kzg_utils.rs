#[cfg(feature = "host")]
pub use rust_kzg::eip_4844::load_trusted_setup_rust;

use rust_kzg::eip_4844::{blob_to_kzg_commitment_rust, load_trusted_setup_string, load_trusted_setup_rust};
use rust_kzg::{
    eip_4844::bytes_to_blob, FFTSettings, Fr, G1Affine, G1Fp, G1GetFp, G1LinComb, G1Mul, Poly, G1, G2,
};
use sha2::{Digest, Sha256};
use rust_ckzg::kzgsettings4844::KzgKZGSettings4844;

pub const VERSIONED_HASH_VERSION_KZG: u8 = 0x01;

pub fn blob_to_kzg_versioned_hash(
    blob: &[u8],
    settings: &KzgKZGSettings4844,
) -> Result<[u8; 32], String> {
    let kzg_blob: Vec<TFr> = bytes_to_blob(blob).map_err(|e| e.to_string())?;
    let hash = blob_to_kzg_commitment_rust(&kzg_blob, settings).unwrap();
    let mut res = Sha256::digest(hash.to_bytes());
    res[0] = VERSIONED_HASH_VERSION_KZG;
    Ok(res.into())
}

pub fn init_kzg_setting(contents: &str) -> Result<KzgKZGSettings4844, String> {
    let (g1_bytes, g2_bytes) = load_trusted_setup_string(&contents)?;
    load_trusted_setup_rust(g1_bytes.as_slice(), g2_bytes.as_slice())
}

#[cfg(test)]
mod test {
    use super::*;
    use kzg_trust_setup::KZG_TRUST_SETUP;

    #[test]
    fn test_blob_to_kzg_versioned_hash() {
        let settings = init_kzg_setting(&KZG_TRUST_SETUP).unwrap();
        let blob = [0; 4096*32];
        let hash = blob_to_kzg_versioned_hash(blob, &settings).unwrap();
        println!("{:?}", hash);
    }
}