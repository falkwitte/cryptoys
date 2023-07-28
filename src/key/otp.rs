//! one-time pad
//! #### NOT ACCURATE
//! This is not a correct one-time pad scheme, as the key size in a one-time pad scheme has to be the same size as the message, 
//! in this one-time pad scheme the key can be of any size, regardless of the message size.
//! <br>
//! An accurate version is in development.

use crate::Solve;

impl Solve for OtpCipher {
    fn solve(&self) -> String {
        let mut plaintext: Vec<u8> = vec![];

        for byte in self.ciphertext.to_string().into_bytes() {
            let mut encrypted_bytes = 0;
            for padding in &self.pad {
                encrypted_bytes = byte ^ padding;
            }
            plaintext.push(encrypted_bytes);
        }

        String::from_utf8(plaintext).unwrap()
    }
}

impl ToString for OtpCipher {
    fn to_string(&self) -> String {
        self.ciphertext.to_string()
    }
}

pub struct OtpCipher {
    ciphertext: String,
    pad: Vec<u8>,
}

// TODO use generics for the type of pad

/// Encrypts text with the otp cipher using a given pad
/// # Example
/// ```
/// use cryptoys::key::otp;
///
/// let pad: Vec<u8> = vec![1, 3, 3, 7];
///
/// assert_eq!("Obkkh", otp::encrypt(pad, "Hello").to_string())
/// ```
pub fn encrypt(pad: Vec<u8>, plaintext: &str) -> OtpCipher {
    let mut ciphertext: Vec<u8> = vec![];

    for byte in plaintext.to_string().into_bytes() {
        let mut encrypted_bytes = 0;
        for padding in &pad {
            encrypted_bytes = byte ^ padding;
        }
        ciphertext.push(encrypted_bytes);
    }

    let ciphertext = String::from_utf8(ciphertext).unwrap();

    OtpCipher {
        ciphertext: ciphertext,
        pad: pad,
    }
}

/// For the readability of code that is written with this library, i am going to include this function.
/// But know that it is redundant.
/// <br>
/// Decrypts otp encrypted text with a given pad
/// # Example
/// ```
/// use cryptoys::key::otp;
///
/// let pad: Vec<u8> = vec![1, 3, 3, 7];
///
/// assert_eq!("Hello", otp::decrypt(pad, "Obkkh"))
///
/// ```
pub fn decrypt(pad: Vec<u8>, ciphertext: &str) -> String {
    let mut plaintext: Vec<u8> = vec![];

    for byte in ciphertext.to_string().into_bytes() {
        let mut encrypted_bytes = 0;
        for padding in &pad {
            encrypted_bytes = byte ^ padding;
        }
        plaintext.push(encrypted_bytes);
    }

    String::from_utf8(plaintext).unwrap()
}
