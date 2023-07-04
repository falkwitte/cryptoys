use crate::historical::caesar;
use crate::historical::common_traits::Solve;

pub struct Rot13Ciphertext{
    ciphertext: String
}

impl Solve for Rot13Ciphertext{
    fn solve(&self) -> String{
        caesar::encrypt(self.ciphertext.as_str(), 13).to_string()
    }
}

impl ToString for Rot13Ciphertext{
    fn to_string(&self) -> String{
        self.ciphertext.to_string()
    }
}

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

pub fn decrypt(ciphertext: &str) -> String{
    caesar::encrypt(ciphertext, 13).to_string()
}

