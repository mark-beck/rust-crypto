use serde::{Deserialize, Serialize};

use strum::{EnumString, IntoStaticStr};
use zeroize::Zeroize;

/// Represents the available hashing algorithms.
///
/// This enum provides a C-compatible representation of various hashing algorithms,
/// including both historically significant and modern, secure algorithms.
///
/// When choosing a hashing algorithm, consider its security level and known vulnerabilities.
/// Algorithms like SHA-1, MD2, MD4, and MD5 are considered insecure for most cryptographic
/// purposes due to practical collision attacks and should be avoided for new applications.
/// Prefer using more secure algorithms like SHA-2 or SHA-3 for cryptographic purposes.
/// flutter_rust_bridge:non_opaque
#[repr(C)]
#[derive(
    Clone,
    Debug,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Default,
    EnumString,
    IntoStaticStr,
    Zeroize,
)]
#[cfg_attr(feature = "ts-interface", derive(ts_rs::TS), ts(export))]
pub enum CryptoHash {
    Sha2_224,
    Sha2_256,
    Sha2_384,
    #[default]
    Sha2_512,
    Sha2_512_224,
    Sha2_512_256,
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    Blake2b,
}
