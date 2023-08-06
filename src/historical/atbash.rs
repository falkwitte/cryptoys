//! Atbash cipher

use crate::utils::{find_key_for_value, text_preprocessor};
use crate::Solve;
use std::collections::HashMap;

pub struct AtbashCiphertext {
    ciphertext: String,
}

impl ToString for AtbashCiphertext {
    fn to_string(&self) -> String {
        self.ciphertext.to_string()
    }
}

/// solves a AtbashCiphertext
impl Solve for AtbashCiphertext {
    fn solve(&self) -> String {
        decrypt(&self.ciphertext)
    }
}

/// encrypts plaintext with the atbash cipher
/// # Example
/// ```
/// use cryptoys::historical::atbash;
///
/// let encrypted = atbash::encrypt("abcdefghijklmnopqrstuvwxyz").to_string();
/// assert_eq!("ZYXWVUTSRQPONMLKJIHGFEDCBA", encrypted)
/// ```
pub fn encrypt(plaintext: &str) -> AtbashCiphertext {
    let plaintext = text_preprocessor(plaintext);

    // map every letter of the alphabet to a number
    let mut alphabet: HashMap<char, i32> = HashMap::new();
    let alph = "abcdefghijklmnopqrstuvwxyz";
    for (index, c) in alph.chars().enumerate() {
        alphabet.insert(c, index.try_into().unwrap());
    }

    // find the value(number) for every char in plaintext
    let text_values: Vec<i32> = plaintext
        .chars()
        .filter(|pc| pc.is_alphabetic())
        .map(|pc| *(alphabet.get(&pc).unwrap()))
        .collect();

    // apply encryption function to values from plaintext
    let text_values: Vec<i32> = text_values
        .iter()
        .map(|value| (-(value + 1) % 26 + 26) % 26)
        .collect();

    // get chars corresponding to new text values
    let new_value: Vec<char> = text_values
        .iter()
        .map(|val| find_key_for_value(&alphabet, *val))
        .collect();

    let ciphertext = new_value.into_iter().collect::<String>().to_uppercase();

    AtbashCiphertext { ciphertext }
}

/// For the readability of code that is written with this library, i am going to include this function.
/// But know that it is redundant.
/// <br>
/// Decrypts a ciphertext encrypted with atbash
/// # Example
/// ```
/// use cryptoys::historical::atbash;
///
/// let decryption = atbash::decrypt("ZYXWVUTSRQPONMLKJIHGFEDCBA");
/// assert_eq!("ABCDEFGHIJKLMNOPQRSTUVWXYZ", decryption)
/// ```
pub fn decrypt(ciphertext: &str) -> String {
    let ciphertext = text_preprocessor(ciphertext);

    // map every letter of the alphabet to a number
    let mut alphabet: HashMap<char, i32> = HashMap::new();
    let alph = "abcdefghijklmnopqrstuvwxyz ";
    for (index, c) in alph.chars().enumerate() {
        alphabet.insert(c, index.try_into().unwrap());
    }

    // find the value(number) for every char in plaintext
    let text_values: Vec<i32> = ciphertext
        .chars()
        .map(|pc| *(alphabet.get(&pc).unwrap()))
        .collect();

    // apply encryption function to values from plaintext
    let text_values: Vec<i32> = text_values
        .iter()
        .map(|value| (-(value + 1) % 26 + 26) % 26)
        .collect();

    // get chars corresponding to new text values
    let new_value: Vec<char> = text_values
        .iter()
        .map(|val| find_key_for_value(&alphabet, *val))
        .collect();

    new_value.into_iter().collect::<String>().to_uppercase()
}
