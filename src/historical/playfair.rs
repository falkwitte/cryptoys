use std::collections::HashMap;
use std::fs::remove_dir;

use crate::{historical::common_traits::Solve};
use crate::utils::{self, remove_whitespace_lowercase};

pub struct PlayfairCiphertext<'a>{
    ciphertext: String,
    key: &'a str
}

pub fn new<'a>(plaintext: &'a str, key: &'a str) -> PlayfairCiphertext<'a>{
    let ciphertext = plaintext.to_string();
    PlayfairCiphertext { ciphertext, key }
}

impl<'a> ToString for PlayfairCiphertext<'a>{
    fn to_string(&self) -> String {
        self.ciphertext.clone()
    }
}


fn preprocess_text(text: &str) -> String{
    let mut processed_text = String::new();

    let text = remove_whitespace_lowercase(text);
    let filtered_text: String = text.chars().filter(|c| c.is_alphabetic()).collect();

    let mut chars_iter = filtered_text.chars().peekable();
    while let Some(c) = chars_iter.next() {
        if c == 'j'{
            processed_text.push('i');
            continue;
        }
        processed_text.push(c);
        if let Some(&next_c) = chars_iter.peek() {
            if c == next_c {
                processed_text.push('x');
            }
        }
    }

    if processed_text.len() % 2 != 0 {
        processed_text.push('x');
    }

    processed_text
}



pub struct Pos{
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

        if c == 'j' || !c.is_alphabetic(){
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

pub fn encrypt(plaintext: &str, key: &str) -> String{
    let key_table = generate_key_table(key);
    let processed_text = preprocess_text(plaintext);
    
    let mut result = String::new();

    let mut char_pairs: Vec<char> = processed_text.chars().collect();
    while !char_pairs.is_empty() || char_pairs.len() == 1{
        let first_char = char_pairs.remove(0);
        println!("{first_char}");
        let second_char = char_pairs.remove(0);
        println!("{second_char}");

        let first_pos = key_table.get(&first_char).unwrap();
        let second_pos = key_table.get(&second_char).unwrap();

        let encrypted_pair = if first_pos.x == second_pos.x{
            // Same column, shift down
            let new_y1 = (first_pos.y + 1) % 5; // modulus to wrap around
            let new_y2 = (second_pos.y + 1) % 5;

            (key_table.iter().find(|(_, p )| p.x == first_pos.x && p.y == new_y1).unwrap().0, 
             key_table.iter().find(|(_,p)| p.x == second_pos.x && p.y == new_y2).unwrap().0)

        }else if first_pos.y == second_pos.y {
            // Same row, shift right
            let new_x1 = (first_pos.x + 1) % 5;
            let new_x2 = (second_pos.x + 1) % 5;

            (key_table.iter().find(|(_, p)| p.x == new_x1 && p.y == first_pos.y).unwrap().0,
             key_table.iter().find(|(_, p)| p.x == new_x2 && p.y == second_pos.y).unwrap().0)
        } else {
            // Rectangle, swap columns
            (key_table.iter().find(|(_, p)| p.x == second_pos.x && p.y == first_pos.y).unwrap().0,
             key_table.iter().find(|(_, p)| p.x == first_pos.x && p.y == second_pos.y).unwrap().0)
        };

        result.push(*encrypted_pair.0);
        result.push(*encrypted_pair.1);
    }
    result
}

#[cfg(test)]
mod playfair_tests{
    use super::*;

    #[test]
    fn test_generate_key_table() {
        let mut test_hashmap: HashMap<char, Pos> = HashMap::new();
        
        let five_grid = "helxloworldxabcfgikmnpqrs";
        let mut x_counter = 0_u8;
        let mut y_counter = 0_u8;
        for c in five_grid.chars(){
            
            test_hashmap.insert(c, Pos{x: x_counter, y: y_counter});
            x_counter += 1;
            
            if x_counter > 4{
                x_counter = 1;
                y_counter +=1;
                
                if y_counter > 4{
                    break
                }
            }
        }
        let hashmap = generate_key_table("helxloworldx");
        let (key, value) = hashmap.get_key_value(&'a').unwrap();        
        let (test_key, test_value) = test_hashmap.get_key_value(&'a').unwrap();

        assert_eq!((test_key, test_value.x), (key, value.x));
    }

    #[test]
    fn test_preprocess_text(){
        assert_eq!("keyilx".to_string(), preprocess_text("key!jl"))
    }

    #[test]
    fn test_encryption(){
        assert_eq!("ea".to_string(), encrypt("he", "he"))
    }
}

