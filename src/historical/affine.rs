//! Affine cipher
//! ## Warning
//! Please keep in mind that this function only encrypts and decrypts using the given values.
//! There are no checks implemented wether these values are correct.
//! E.g. there is no check to verify that a and m are coprime.
//! If there is any interrest, then such checks might be implemented in the future.

use crate::utils::find_key_for_value;
use std::collections::HashMap;

pub fn encrypt(a: i32, b: i32, plaintext: String) -> Result<String, &'static str> {
    // map every letter of the alphabet to a number
    let mut alphabet: HashMap<char, i32> = HashMap::new();
    let alph = "abcdefghijklmnopqrstuvwxyz";
    for (index, c) in alph.chars().enumerate() {
        alphabet.insert(c, index.try_into().unwrap());
    }

    // find the value(number) for every char in plaintext
    let text_values: Vec<i32> = plaintext
        .chars()
        .map(|pc| *(alphabet.get(&pc).unwrap()))
        .collect();

    for i in 0..plaintext.len() {
        let _math: i32 = (a * i as i32 + b) % plaintext.len() as i32;
    }

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

    Ok(new_value.into_iter().collect::<String>())
}
