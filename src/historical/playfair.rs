use std::collections::HashMap;

use crate::utils::remove_whitespace_lowercase;

fn preprocess_text(text: &str) -> String {
    let mut processed_text = String::new();

    let text = remove_whitespace_lowercase(text);
    let filtered_text: String = text.chars().filter(|c| c.is_alphabetic()).collect();

    let mut chars_iter = filtered_text.chars().peekable();
    while let Some(c) = chars_iter.next() {
        if c == 'j' {
            processed_text.push('i');
            continue;
        }
        processed_text.push(c);
        if let Some(&next_c) = chars_iter.peek() {
            if c == next_c {
                processed_text.push('a');
            }
        }
    }

    if processed_text.len() % 2 != 0 {
        processed_text.push('a');
    }

    processed_text
}

pub struct Pos {
    x: u8,
    y: u8,
}

/// function to generate 5x5 key square
fn generate_key_table(key: &str) -> HashMap<char, Pos> {
    // a hashmap to store the position of every letter in
    let mut pos: HashMap<char, Pos> = HashMap::new();

    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    let mut x_counter = 0_u8;
    let mut y_counter = 0_u8;

    for c in key.chars() {
        if c == 'j' || !c.is_alphabetic() {
            continue;
        }

        if pos.contains_key(&c) {
            continue;
        }

        pos.insert(
            c,
            Pos {
                x: x_counter,
                y: y_counter,
            },
        );
        x_counter += 1;
        if x_counter > 4 {
            x_counter = 0;
            y_counter += 1;
        }
    }

    for c in alphabet.chars() {
        if c == 'j' || key.contains(c) {
            continue;
        }

        pos.insert(
            c,
            Pos {
                x: x_counter,
                y: y_counter,
            },
        );
        x_counter += 1;

        if x_counter > 4 {
            x_counter = 0;
            y_counter += 1;

            if y_counter > 4 {
                return pos;
            }
        }
    }
    pos
}


/// encrypts plaintext with the playfair cipher using a key
/// 
/// # Example
/// ```
/// use cryptoys::historical::playfair; 
/// 
/// fn test_playfair_encryption() {
///      assert_eq!("KBWEDR".to_string(), playfair::encrypt("hello", "world"))
/// }
/// ```
pub fn encrypt(plaintext: &str, key: &str) -> String {
    let key_table = generate_key_table(key);
    let processed_text = preprocess_text(plaintext);

    let mut result = String::new();

    let mut char_pairs: Vec<char> = processed_text.chars().collect();
    while !char_pairs.is_empty() || char_pairs.len() == 1 {
        let first_char = char_pairs.remove(0);
        let second_char = char_pairs.remove(0);

        let first_pos = key_table.get(&first_char).unwrap();
        let second_pos = key_table.get(&second_char).unwrap();

        let encrypted_pair = if first_pos.x == second_pos.x {
            // Same column, shift down
            let new_y1 = (first_pos.y + 1) % 5; // modulus to wrap around
            let new_y2 = (second_pos.y + 1) % 5;

            (
                key_table
                    .iter()
                    .find(|(_, p)| p.x == first_pos.x && p.y == new_y1)
                    .unwrap()
                    .0,
                key_table
                    .iter()
                    .find(|(_, p)| p.x == second_pos.x && p.y == new_y2)
                    .unwrap()
                    .0,
            )
        } else if first_pos.y == second_pos.y {
            // Same row, shift right
            let new_x1 = (first_pos.x + 1) % 5;
            let new_x2 = (second_pos.x + 1) % 5;

            (
                key_table
                    .iter()
                    .find(|(_, p)| p.x == new_x1 && p.y == first_pos.y)
                    .unwrap()
                    .0,
                key_table
                    .iter()
                    .find(|(_, p)| p.x == new_x2 && p.y == second_pos.y)
                    .unwrap()
                    .0,
            )
        } else {
            // Rectangle, swap columns
            (
                key_table
                    .iter()
                    .find(|(_, p)| p.x == second_pos.x && p.y == first_pos.y)
                    .unwrap()
                    .0,
                key_table
                    .iter()
                    .find(|(_, p)| p.x == first_pos.x && p.y == second_pos.y)
                    .unwrap()
                    .0,
            )
        };

        result.push(*encrypted_pair.0);
        result.push(*encrypted_pair.1);
    }
    result.to_uppercase()
}

/// decrypts text that was encrypted with the playfair cipher using a key
/// 
/// # Encrypt
/// ```
/// use cryptoys::historical::playfair;
/// 
/// fn test_playfair_decryption() {
///     assert_eq!("hello".to_string(), playfair::decrypt("kbwedr", "world"))
/// }
/// ```
pub fn decrypt(encrypted_text: &str, key: &str) -> String {
    let key_table = generate_key_table(key);
    let processed_text = preprocess_text(encrypted_text);

    let mut result = String::new();

    let mut char_pairs: Vec<char> = processed_text.chars().collect();
    while !char_pairs.is_empty() || char_pairs.len() == 1 {
        let first_char = char_pairs.remove(0);
        let second_char = char_pairs.remove(0);

        let first_pos = key_table.get(&first_char).unwrap();
        let second_pos = key_table.get(&second_char).unwrap();

        let encrypted_pair = if first_pos.x == second_pos.x {
            // Same column, shift down
            let new_y1 = (first_pos.y - 1) % 5; // modulus to wrap around
            let new_y2 = (second_pos.y - 1) % 5;

            (
                key_table
                    .iter()
                    .find(|(_, p)| p.x == first_pos.x && p.y == new_y1)
                    .unwrap()
                    .0,
                key_table
                    .iter()
                    .find(|(_, p)| p.x == second_pos.x && p.y == new_y2)
                    .unwrap()
                    .0,
            )
        } else if first_pos.y == second_pos.y {
            // Same row, shift right
            let new_x1 = (first_pos.x - 1) % 5;
            let new_x2 = (second_pos.x - 1) % 5;

            (
                key_table
                    .iter()
                    .find(|(_, p)| p.x == new_x1 && p.y == first_pos.y)
                    .unwrap()
                    .0,
                key_table
                    .iter()
                    .find(|(_, p)| p.x == new_x2 && p.y == second_pos.y)
                    .unwrap()
                    .0,
            )
        } else {
            // Rectangle, swap columns
            (
                key_table
                    .iter()
                    .find(|(_, p)| p.x == second_pos.x && p.y == first_pos.y)
                    .unwrap()
                    .0,
                key_table
                    .iter()
                    .find(|(_, p)| p.x == first_pos.x && p.y == second_pos.y)
                    .unwrap()
                    .0,
            )
        };

        result.push(*encrypted_pair.0);
        result.push(*encrypted_pair.1);
    }

    let result: String = result.chars().filter(|c| *c != 'a').collect();

    result
}

#[cfg(test)]
mod playfair_tests {
    use super::*;

    #[test]
    fn test_playfair_generate_key_table() {
        let mut test_hashmap: HashMap<char, Pos> = HashMap::new();

        let five_grid = "playfirexmbcdghknoqstuvwz";
        let mut x_counter = 0_u8;
        let mut y_counter = 0_u8;
        for c in five_grid.chars() {
            test_hashmap.insert(
                c,
                Pos {
                    x: x_counter,
                    y: y_counter,
                },
            );
            x_counter += 1;

            if x_counter > 4 {
                x_counter = 0;
                y_counter += 1;

                if y_counter > 4 {
                    break;
                }
            }
        }
        let hashmap = generate_key_table("playfaire example");
        let (key, value) = hashmap.get_key_value(&'u').unwrap();
        let (test_key, test_value) = test_hashmap.get_key_value(&'u').unwrap();

        assert_eq!((test_key, test_value.x), (key, value.x));
    }

    #[test]
    fn test_playfair_preprocess_text() {
        assert_eq!("keyila".to_string(), preprocess_text("key!jl"))
    }

    #[test]
    fn test_playfair_encryption() {
        assert_eq!("KBWEDR".to_string(), encrypt("hello", "world"))
    }

    #[test]
    fn test_playfair_decryption() {
        assert_eq!("hello".to_string(), decrypt("kbwedr", "world"))
    }
}
