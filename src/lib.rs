pub mod hyrax;
pub mod ligero;
pub mod merkle_tree;
pub mod mle;
mod ntt;
pub mod poseidon;
mod transcript;

pub use crate::{
    ntt::{intt, ntt},
    transcript::{Prover, Sponge, Verifier},
};
use {ark_bn254::Fr, ark_ff::Zero, hyrax::HyraxCommiter, rand::Rng};

pub fn prove_r1cs(
    rng: &mut impl Rng,
    transcript: &mut Prover,
    size: usize,
    _a: &[(usize, usize, Fr)],
    _b: &[(usize, usize, Fr)],
    _c: &[(usize, usize, Fr)],
    z: &[Fr],
) {
    let hyrax = HyraxCommiter::new(size);

    // Commit to z
    let _z_commitment = hyrax.commit(rng, transcript, z);

    // Compute A ⋅ z, B ⋅ z, C ⋅ z
    let _az = vec![Fr::zero(); size];
    let _bz = vec![Fr::zero(); size];
    let _cz = vec![Fr::zero(); size];

    // Compute MLE of eq(r, x)
    let _e = todo!();

    // Prove the sum equals zero
    // let (r, rs) = prove_sumcheck_r1cs(transcript, size, e, az, bz, cz, Fr::zero());

    // Random linear combination of Az, Bz, Cz

    // Prove M ⋅ z
    // let (r, rs) = prove_sumcheck_product(transcript, size, f, g, sum);
}
