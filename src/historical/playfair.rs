use crate::historical::common_traits::Solve;

pub struct PlayfairCiphertext<'a>{
    ciphertext: String,
    key: &'a str
}

    pub fn new<'a>(plaintext: &'a str, key: &'a str) -> PlayfairCiphertext<'a>{
        let ciphertext = plaintext.to_string();
        PlayfairCiphertext { ciphertext, key }
    }

impl<'a> Solve for PlayfairCiphertext<'a>{
    fn solve(&self) -> String {
        self.ciphertext.clone()     
    }
}

impl<'a> ToString for PlayfairCiphertext<'a>{
    fn to_string(&self) -> String {
        self.ciphertext.clone()
    }
}