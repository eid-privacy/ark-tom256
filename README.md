# Tom-256

This library implements the tom256 curve and is a copy of the secp256r1 implementation.
Parameters source: <https://neuromancer.sk/std/other/Tom-256#>
Paper: <https://eprint.iacr.org/2021/1183>

The tom256 curve is interesting because the order of tom256 equals the field size of
secp256r1. This allows to take points from secp256r1 and encode them as scalars
in tom256, map them to tom256 points, and do point arithmetics in tom256, without having
to emulate the modulo in tom256, which is a very expensive operation in ZKP.

The field size of tom256 is bigger than the order of secp256r1 to make sure that all
points from secp256r1 map to tom256.

Using the notation from arkworks:
* $t256_q > secp256r1_r$
* $secp256r1_q = t256_r$.

Using the notation from neuromancer.sk:
* $t256_p > secp256r1_n$
* $secp256r1_p = t256_n$.

Curve information - in parentheses names from neuromancer.sk:
* Base field: q (p) =
  * secp: 115792089210356248762697446949407573530086143415290314195533631308867097853951
  * t256: 115792089210356248762697446949407573530594504085698471288169790229257723883799
* Scalar field: r (n) =
  * secp: 115792089210356248762697446949407573529996955224135760342422259061068512044369
  * t256: 115792089210356248762697446949407573530086143415290314195533631308867097853951
* Curve equation a (a) =
  * secp: 0
  * t256: 115792089210356248762697446949407573530594504085698471288169790229257723883796
* Curve equation b (b) =
  * secp: 7
  * t256: 81531206846337786915455327229510804132577517753388365729879493166393691077718
* Base point G =
  * secp: (55066263022277343669578718895168534326250603453777594175500187360389116729240,
         32670510020758816978083085130507043184471273380659243275938904335757337482424)
  * t256: (3, 40902200210088653215032584946694356296222563095503428277299570638400093548589)
* Curve equation: y^2 = x^3 + ax + b

# Development

## Setup

Run once after cloning to activate the git hooks (requires [devbox](https://www.jetify.com/devbox)):

```
devbox run setup
```

## Testing

The R1CS constraint tests require the `r1cs` feature:

```
cargo test --features r1cs,zero-flag
```

## Using this library in another project

The `zero-flag` feature (enabled by default) uses `type ZeroFlag = ()` in the `SWCurveConfig` impl, which requires a patched version of `ark-ec` not yet released on crates.io.

### With the patched `ark-ec` (recommended)

Add the same patch to your root `Cargo.toml`:

```toml
[patch.crates-io]
ark-ec = { git = "https://github.com/arkworks-rs/algebra" }
```

Then depend on this library normally:

```toml
[dependencies]
ark-tom256 = "..."
```

### Without the patched `ark-ec`

Disable the default features to exclude `zero-flag`:

```toml
[dependencies]
ark-tom256 = { version = "...", default-features = false }
```

Re-enable any other default features you need (e.g. `std`):

```toml
ark-tom256 = { version = "...", default-features = false, features = ["std"] }
```

The `curve-constraint-tests/` directory contains a vendored copy of
[`ark-curve-constraint-tests`](https://github.com/arkworks-rs/algebra/tree/master/curves/curve-constraint-tests)
from the arkworks algebra repository. It is not published on crates.io, so it
is kept here to make the repository self-contained.

# Thanks

Thanks to @lovesh (Lovesh Harchandani) for helping us with this.
