use crate::historical::caesar;
use crate::historical::common_traits::Solve;

pub struct Rot13Ciphertext{
    ciphertext: String
}

/// Solves a Rot13Ciphertext
impl Solve for Rot13Ciphertext{
    fn solve(&self) -> String{
        caesar::encrypt(self.ciphertext.as_str(), 13).to_string()
    }
}

/// Returns a string from a Rot13Ciphertext
impl ToString for Rot13Ciphertext{
    fn to_string(&self) -> String{
        self.ciphertext.to_string()
    }
}

/// encrypts plaintext using rot13
/// 
/// # Example
/// ```
/// use cryptoys::historical::rot13;
/// 
/// let rot13_encrypted_text = rot13::encrypt("Hello World!");
/// 
/// assert_eq!("Uryyb Jbeyq!".to_string(), rot13_encrypted_text.to_string());
/// ```
pub fn encrypt(plaintext: &str) -> Rot13Ciphertext{


    let mut result = String::new();

    for c in plaintext.chars(){
        let new_char = match c {
                'A'..='Z' => (((c as u32 - 'A' as u32 + 13_u32) % 26) + 'A' as u32) as u8 as char,
                'a'..='z' => (((c as u32 - 'a' as u32 + 13_u32) % 26) + 'a' as u32) as u8 as char,
                _ => c,
        };
        result.push(new_char);
    }

    Rot13Ciphertext { ciphertext: result }
}


/// decrypts rot13 ciphertext
/// 
/// # Example
/// ```
/// use cryptoys::historical::rot13;
/// 
/// let decrypt_rot13_text = rot13::decrypt("Uryyb Jbeyq!");
/// 
/// assert_eq!("Hello World!".to_string(), decrypt_rot13_text);
/// ```
pub fn decrypt(ciphertext: &str) -> String{
    caesar::encrypt(ciphertext, 13).to_string()
}

