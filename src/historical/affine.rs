//! Affine cipher
//! ## Warning
//! Please keep in mind that this function only encrypts and decrypts using the given values.
//! There are no checks implemented wether these values are correct.
//! E.g. there is no check to verify that a and m are coprime.
//! If there is any interrest, then such checks might be implemented in the future.

use crate::utils::{find_key_for_value, text_preprocessor};
use crate::Solve;
use modinverse::modinverse;
use std::collections::HashMap;

pub struct AffineCiphertext {
    ciphertext: String,
    a: i32,
    b: i32,
}

/// solves a AffineCiphertext
impl Solve for AffineCiphertext {
    fn solve(&self) -> String {
        decrypt(self.a, self.b, &self.ciphertext)
    }
}

impl ToString for AffineCiphertext {
    fn to_string(&self) -> String {
        self.ciphertext.to_string()
    }
}

/// Encrypts plaintext using the affine cipher with given numbers a and b
/// <br>
/// Note that a and b need to be coprime.
/// # Example
/// ```
/// use cryptoys::historical::affine;
/// let encrypted = affine::encrypt(5, 8, "AFFINE cipher");
/// assert_eq!("IHHWVCSWFRCP".to_string(), encrypted.to_string())
/// ```
pub fn encrypt(a: i32, b: i32, plaintext: &str) -> AffineCiphertext {
    let plaintext = text_preprocessor(plaintext);

    // map every letter of the alphabet to a number
    let mut alphabet: HashMap<char, i32> = HashMap::new();
    let alph = "abcdefghijklmnopqrstuvwxyz ";
    for (index, c) in alph.chars().enumerate() {
        alphabet.insert(c, index.try_into().unwrap());
    }

    // find the value(number) for every char in plaintext
    let text_values: Vec<i32> = plaintext
        .chars()
        .map(|pc| *(alphabet.get(&pc).unwrap()))
        .collect();

    // apply encryption function to values from plaintext
    let text_values: Vec<i32> = text_values
        .iter()
        .map(|value| (a * value + b) % 26)
        .collect();

    // get chars corresponding to new text values
    let new_value: Vec<char> = text_values
        .iter()
        .map(|val| find_key_for_value(&alphabet, *val))
        .collect();

    let ciphertext = new_value.into_iter().collect::<String>().to_uppercase();
    AffineCiphertext { ciphertext, a, b }
}

/// Decrypts ciphertext encrypted with the affine cipher
/// # Example
/// ```
/// use cryptoys::historical::affine;
/// let decryption = affine::decrypt(5, 8,"IHHWVCSWFRCP");
/// assert_eq!("AFFINECIPHER", decryption)
/// ```
pub fn decrypt(a: i32, b: i32, ciphertext: &str) -> String {
    let ciphertext = text_preprocessor(ciphertext);
    let a_modinverse = modinverse(a, 26).unwrap();

    // map every letter of the alphabet to a number
    let mut alphabet: HashMap<char, i32> = HashMap::new();
    let alph = "abcdefghijklmnopqrstuvwxyz";
    let mut index = 0;
    for c in alph.chars() {
        alphabet.insert(c, index);
        index += 1;
    }

    // find the value(number) for every char in ciphertext
    let text_values: Vec<i32> = ciphertext
        .chars()
        .map(|pc| *(alphabet.get(&pc).unwrap()))
        .collect();

    // apply decryption function to values from ciphertext
    let text_values: Vec<i32> = text_values
        .iter()
        .map(|value| ((a_modinverse * (value - b) % 26 + 26) % 26))
        .collect();

    // get chars corresponding to new text values
    let new_value: Vec<char> = text_values
        .iter()
        .map(|val| find_key_for_value(&alphabet, *val))
        .collect();

    new_value.into_iter().collect::<String>().to_uppercase()
}
