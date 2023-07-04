use cryptoys::historical::{common_traits::{Solve, ToString}, caesar};

/// encrypted values are taken from https://cryptii.com/pipes/caesar-cipher

#[test]
fn caesarcipher_encryption_test(){
    assert_eq!(
        String::from("Khoor Zruog!"),
        caesar::encrypt("Hello World!", 3).to_string()
    )
}

#[test]
fn caesarcipher_decryption_test(){
    assert_eq!(
        String::from("Hello World!"),
        caesar::decrypt("Khoor Zruog!", 3)
    )
}

#[test]
fn caesarcipher_solve_test(){

    let encrypted = caesar::encrypt("Hello World!", 3);

    assert_eq!(
        String::from("Hello World!"),
        encrypted.solve()
    )

}

use cryptoys::historical::rot13;

#[test]
fn rot13_encryption_test(){
    assert_eq!(
        String::from("Uryyb Jbeyq!"),
        rot13::encrypt("Hello World!").to_string()
    )
}

#[test]
fn rot13_decryption_test(){
    assert_eq!(
        String::from("Hello World!"),
        rot13::decrypt("Uryyb Jbeyq!")
    )
}

#[test]
fn rot13_solve_test(){
    let encrypted = rot13::encrypt("Hello World!");

    assert_eq!(
        String::from("Hello World!"),
        encrypted.solve()        
    )
}