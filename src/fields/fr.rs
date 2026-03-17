use ark_ff::fields::{Fp256, MontBackend, MontConfig};

// This corresponds to the modulo of the
// github.com:arkworks-rs/algebra/curves/secp256r1/src/fields/fq.rs
#[derive(MontConfig)]
#[modulus = "115792089210356248762697446949407573530086143415290314195533631308867097853951"]
#[generator = "6"]
pub struct FrConfig;
pub type Fr = Fp256<MontBackend<FrConfig, 4>>;
