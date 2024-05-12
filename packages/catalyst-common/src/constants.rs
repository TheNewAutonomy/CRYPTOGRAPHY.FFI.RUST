//! Common constants such as buffer sizes for keys and signatures.

/// The length of a ed25519 `Signature`, in bytes.
pub const SIGNATURE_LENGTH: usize = 64;

/// The length of a ed25519ph `PrivateKey`, in bytes.
pub const PRIVATE_KEY_LENGTH: usize = 32;

/// The length of an ed25519ph `VerifyingKey`, in bytes.
pub const PUBLIC_KEY_LENGTH: usize = 32;

/// The max of the ed25519ph context, in bytes.
pub const CONTEXT_MAX_LENGTH: usize = 255;
