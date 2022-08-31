//! Library crate that provides low-level functionality needed to establish TLS
//! connections.
//!
//! In particular, the crate provides functionality to
//! * generate TLS key material and wrap the public part in an X.509 certificate

#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]

pub mod keygen;
