use std::collections::HashMap;

use crate::{historical::common_traits::Solve, utils};

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

// nothing existed here

struct Pos{
    x: u8,
    y: u8,
}

/// function to generate 5x5 key square
fn generate_key_table(key: &str) -> HashMap<char, Pos>{
    
    // a hashmap to store the position of every letter in
    let mut pos: HashMap<char, Pos> = HashMap::new();

    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    let mut x_counter = 0_u8;
    let mut y_counter = 0_u8;

    for c in key.chars(){

        if c == 'j'{
            continue;
        }

        pos.insert(c, Pos { x: x_counter, y: y_counter });
        x_counter += 1;
        if x_counter > 4{
            x_counter = 1;
            y_counter += 1;
        }
    }
    
    for c in alphabet.chars(){
        
        if c == 'j' || key.contains(c){
            continue;
        }

        pos.insert(c, Pos{ x: x_counter, y: y_counter});
        x_counter += 1;

        if x_counter > 4{
            x_counter = 1;
            y_counter += 1;

            if y_counter > 4 {
                return pos
            }
        }
    }
    pos
}


#[cfg(test)]
mod playfair_tests{
    use super::*;

    #[test]
    fn test_generate_key_table() {
        let mut test_hashmap: HashMap<char, Pos> = HashMap::new();
        
        let five_grid = "keyabcdfghilmnopqrstuvwxz";
        let mut x_counter = 1_u8;
        let mut y_counter = 1_u8;
        for c in five_grid.chars(){
            
            test_hashmap.insert(c, Pos{x: x_counter, y: y_counter});
            x_counter += 1;
            
            if x_counter > 5{
                x_counter = 1;
                y_counter +=1;
                
                if y_counter > 5{
                    break
                }
            }
        }
        let hashmap = generate_key_table("key");
        let (key, value) = hashmap.get_key_value(&'a').unwrap();        
        let (test_key, test_value) = test_hashmap.get_key_value(&'a').unwrap();

        assert_eq!((test_key, test_value.x), (key, value.x));
    }
}
