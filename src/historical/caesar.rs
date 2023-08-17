//! Caesar cipher
use crate::Solve;

pub struct CaesarCiphertext {
    ciphertext: String,
    shift: u8,
}

/// Solves a CaesarCiphertext
impl Solve for CaesarCiphertext {
    fn solve(&self) -> String {
        decrypt(&self.ciphertext, self.shift)
    }
}

/// Returns the encrypted text of a CaesarCiphertext as String
impl ToString for CaesarCiphertext {
    fn to_string(&self) -> String {
        self.ciphertext.clone()
    }
}

/// Encrypts an &str using the caesar cipher with a shift value.
///
/// # Example
/// ```
/// use cryptoys::historical::caesar;
/// let caesarcipher = caesar::encrypt("Hello World!", 10);
///
/// assert_eq!("Rovvy Gybvn!".to_string(), caesarcipher.to_string());     
/// ```
pub fn encrypt(plaintext: &str, shift: u8) -> CaesarCiphertext {
    let mut result = String::new();

    for c in plaintext.chars() {
        let new_char = match c {
            'A'..='Z' => (((c as u32 - 'A' as u32 + shift as u32) % 26) + 'A' as u32) as u8 as char,
            'a'..='z' => (((c as u32 - 'a' as u32 + shift as u32) % 26) + 'a' as u32) as u8 as char,
            _ => c,
        };
        result.push(new_char);
    }
    CaesarCiphertext {
        ciphertext: result,
        shift,
    }
}

/// Decrypts a with the caesar cipher encrypted &str with the original shift value
///
/// # Example
/// ```
/// use cryptoys::historical::caesar;
///
/// let decrypted_text = caesar::decrypt("Rovvy Gybvn!", 10);
///
/// assert_eq!("Hello World!".to_string(), decrypted_text);
/// ```
pub fn decrypt(ciphertext: &str, shift: u8) -> String {
    let decrypt_shift = 26 - (shift % 26);

    encrypt(ciphertext, decrypt_shift).to_string()
}
