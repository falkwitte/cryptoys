//! Affine cipher
//! ## Warning
//! Please keep in mind that this function only encrypts and decrypts using the given values.
//! There are no checks implemented wether these values are correct.
//! E.g. there is no check to verify that a and m are coprime.
//! If there is any interrest, then such checks might be implemented in the future.

use std::collections::HashMap;


pub fn encrypt(a: i32, b: i32, plaintext: String) -> Result<String, &'static str>{

    // map every letter of the alphabet to a number
    let mut alphabet : HashMap<char, i32> = HashMap::new();
    let alph = "abcdefghijklmnopqrstuvwxyz";
    let mut index = 0;
    for c in alph.chars(){
        alphabet.insert(c, index);
        index += 1;
    }

    // find the value(number) for every char in plaintext
    let text_values: Vec<i32> = plaintext.chars().map(|pc|*(alphabet.get(&pc).unwrap())).collect();

    for i in 0..plaintext.len(){
        let math: i32 = (a*i as i32 +b)%plaintext.len() as i32; 
    }

    // apply encryption function to values from plaintext
    let text_values: Vec<i32> = text_values.iter().map(|value | (a*value+b)%26).collect(); 

    // get chars corresponding to new text values
    let new_value = text_values.iter().map(||);



    Ok("hi".to_string())
}