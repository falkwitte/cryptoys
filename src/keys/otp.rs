//TODO implement the one-time pad cipher
use crate::Solve;

impl Solve for OtpCipher{
    fn solve(&self) -> String {
        todo!()
    }
}

impl ToString for OtpCipher{
    fn to_string(&self) -> String {
        self.ciphertext.to_string()
    }
}

pub struct OtpCipher{
    ciphertext: String,
    pad: Vec<u8>,
}

pub fn encrypt(pad: Vec<u8>, plaintext: &str) -> OtpCipher{
    todo!()
}

pub fn decrypt(pad: Vec<u8>, ciphertext: &str) -> String{
    todo!()
}
