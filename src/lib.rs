//! Cryptoys is a cryptographic library that contains cryptographic toys, like historical algorithms(rot13, playfair)
//! or hashing functions like md5.
//! <br>
//! <br>
//! The primary goal of this crate is not to make a fully functioning, save and secure cryptography library (others have done this already),
//! but to provide fun toys to play around with.
//!
//! ##### Features:
//! - historical ciphers
//!     - playfair
//!     - rot13
//!     - caesar
//! - key encryption
//!     - otp

pub mod historical;
pub mod key;
mod utils;

/// Solves a Ciphtertext
pub trait Solve {
    fn solve(&self) -> String;
}
