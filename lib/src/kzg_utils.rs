use c_kzg::{KzgCommitment, BYTES_PER_G1_POINT, BYTES_PER_G2_POINT};
use hex;
use sha2::{Digest, Sha256};

/// Number of G1 Points.
pub const NUM_G1_POINTS: usize = 4096;

/// Number of G2 Points.
pub const NUM_G2_POINTS: usize = 65;

/// A newtype over list of G1 point from kzg trusted setup.
#[derive(Debug, Clone, PartialEq)]
#[repr(transparent)]
pub struct G1Points(pub [[u8; BYTES_PER_G1_POINT]; NUM_G1_POINTS]);

impl Default for G1Points {
    fn default() -> Self {
        Self([[0; BYTES_PER_G1_POINT]; NUM_G1_POINTS])
    }
}

/// A newtype over list of G2 point from kzg trusted setup.
#[derive(Debug, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct G2Points(pub [[u8; BYTES_PER_G2_POINT]; NUM_G2_POINTS]);

impl Default for G2Points {
    fn default() -> Self {
        Self([[0; BYTES_PER_G2_POINT]; NUM_G2_POINTS])
    }
}

#[derive(Debug)]
pub enum KzgErrors {
    /// Failed to get current directory.
    FailedCurrentDirectory,
    /// The specified path does not exist.
    PathNotExists,
    /// Problems related to I/O.
    IOError,
    /// Not a valid file.
    NotValidFile,
    /// File is not properly formatted.
    FileFormatError,
    /// Not able to parse to usize.
    ParseError,
    /// Number of points does not match what is expected.
    MismatchedNumberOfPoints,
}

/// Parses the contents of a KZG trusted setup file into a list of G1 and G2 points.
///
/// These can then be used to create a KZG settings object with
/// [`KzgSettings::load_trusted_setup`](c_kzg::KzgSettings::load_trusted_setup).
pub fn parse_kzg_trusted_setup(trusted_setup: &str) -> Result<(G1Points, G2Points), KzgErrors> {
    let mut lines = trusted_setup.lines();

    // load number of points
    let n_g1 = lines
        .next()
        .ok_or(KzgErrors::FileFormatError)?
        .parse::<usize>()
        .map_err(|_| KzgErrors::ParseError)?;
    let n_g2 = lines
        .next()
        .ok_or(KzgErrors::FileFormatError)?
        .parse::<usize>()
        .map_err(|_| KzgErrors::ParseError)?;

    if n_g1 != NUM_G1_POINTS {
        return Err(KzgErrors::MismatchedNumberOfPoints);
    }

    if n_g2 != NUM_G2_POINTS {
        return Err(KzgErrors::MismatchedNumberOfPoints);
    }

    // load g1 points
    let mut g1_points = G1Points::default();
    for bytes in &mut g1_points.0 {
        let line = lines.next().ok_or(KzgErrors::FileFormatError)?;
        hex::decode_to_slice(line, bytes).map_err(|_| KzgErrors::ParseError)?;
    }

    // load g2 points
    let mut g2_points = G2Points::default();
    for bytes in &mut g2_points.0 {
        let line = lines.next().ok_or(KzgErrors::FileFormatError)?;
        hex::decode_to_slice(line, bytes).map_err(|_| KzgErrors::ParseError)?;
    }

    if lines.next().is_some() {
        return Err(KzgErrors::FileFormatError);
    }

    Ok((g1_points, g2_points))
}

pub const VERSIONED_HASH_VERSION_KZG: u8 = 0x01;

pub fn kzg_to_versioned_hash(commitment: &KzgCommitment) -> [u8; 32] {
    let mut res = Sha256::digest(commitment.as_slice());
    res[0] = VERSIONED_HASH_VERSION_KZG;
    res.into()
}
