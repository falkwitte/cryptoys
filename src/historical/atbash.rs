use std::collections::HashMap;

pub struct AtbashCiphertext{
    ciphertext: String,
}

impl ToString for AtbashCiphertext{
    fn to_string(&self) -> String{
        self.ciphertext.to_string()
    }
}

pub fn encrypt(plaintext: &str) -> AtbashCiphertext{

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

    todo!()

}
