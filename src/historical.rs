#[allow(unused)]

pub trait Solve{
    /// solves a Caesarcipher by shifting the letters back by a u32 number 
    fn solve(&self) -> String;
}

pub trait ToString{
    fn to_string(&self) -> String;
}

//Caesarcipher -----------------------------------------------------------------------
pub struct Caesarcipher{}

pub struct CaesarCiphertext{
    ciphertext: String,
    shift: u8,
}

impl Caesarcipher{
    /// Creates a new Caesarcipher by shifting plaintext by a u32 number
    pub fn new<'a>(plaintext: &'a str, shift: u8) -> CaesarCiphertext{
            let mut result = String::new(); 

            for c in plaintext.chars(){
                let new_char = match c{
                    'A'..='Z' => (((c as u32 - 'A' as u32 + shift as u32) % 26) + 'A' as u32) as u8 as char,
                    'a'..='z' => (((c as u32 - 'a' as u32 + shift as u32) % 26) + 'a' as u32) as u8 as char,
                    _ => c,
                };
                result.push(new_char);
            }
        CaesarCiphertext{ciphertext: result, shift}
    }

    pub fn decipher<'a>(ciphertext: &'a str, shift: u8) -> String{
        let decrypt_shift = 26 - (shift % 26);

        Caesarcipher::new(ciphertext, decrypt_shift).to_string()
    }
}


impl Solve for CaesarCiphertext{
    fn solve(&self) -> String{
        let decrypt_shift = 26 - (self.shift % 26);

        Caesarcipher::new(self.ciphertext.as_str(), decrypt_shift).to_string()
    }
}

impl ToString for CaesarCiphertext{
    fn to_string(&self) -> String{
        self.ciphertext.clone()
    }
}

//-----------------------------------------------------------------------------------

// rot13-----------------------------------------------------------------------------

pub struct Rot13{}

pub struct Rot13Ciphertext{
    ciphertext: String
}

impl Rot13{
    pub fn new<'a>(plaintext: &'a str) -> Rot13Ciphertext{

        let mut result = String::new();

        for c in plaintext.chars(){
            let new_char = match c {
                    'A'..='Z' => (((c as u32 - 'A' as u32 + 13 as u32) % 26) + 'A' as u32) as u8 as char,
                    'a'..='z' => (((c as u32 - 'a' as u32 + 13 as u32) % 26) + 'a' as u32) as u8 as char,
                    _ => c,
            };
            result.push(new_char);
        }

        Rot13Ciphertext { ciphertext: result }
    }

    pub fn decipher<'a>(ciphertext: &'a str) -> String{
        Caesarcipher::new(ciphertext, 13).to_string()
    }
}

impl Solve for Rot13Ciphertext{
    fn solve(&self) -> String{
        Caesarcipher::new(self.ciphertext.as_str(), 13).to_string()
    }
}

impl ToString for Rot13Ciphertext{
    fn to_string(&self) -> String{
        self.ciphertext.to_string()
    }
}

//-----------------------------------------------------------------------------------