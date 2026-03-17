use ark_algebra_bench_templates::*;
use ark_secp256r1::{fq::Fq, fr::Fr, Projective as G};

bench!(
    Name = "Tom256",
    Group = G,
    ScalarField = Fr,
    PrimeBaseField = Fq,
);
