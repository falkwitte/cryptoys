use cryptoys::historical::{Caesarcipher, ToString, Solve};

/// encrypted values are taken from https://cryptii.com/pipes/caesar-cipher

#[test]
fn caesarcipher_encryption_test(){
    assert_eq!(
        String::from("Khoor Zruog!"),
        Caesarcipher::new("Hello World!", 3).to_string()
    )
}

#[test]
fn caesarcipher_decryption_test(){
    assert_eq!(
        String::from("Hello World!"),
        Caesarcipher::decrypt("Khoor Zruog!", 3)
    )
}

#[test]
fn caesarcipher_solve_test(){

    let encrypted = Caesarcipher::new("Hello World!", 3);

    assert_eq!(
        String::from("Hello World!"),
        encrypted.solve()
    )

}

use cryptoys::historical::{Rot13};

#[test]
fn rot13_encryption_test(){
    assert_eq!(
        String::from("Uryyb Jbeyq!"),
        Rot13::new("Hello World!").to_string()
    )
}

#[test]
fn rot13_decryption_test(){
    assert_eq!(
        String::from("Hello World!"),
        Rot13::decrypt("Uryyb Jbeyq!")
    )
}

#[test]
fn rot13_solve_test(){
    let encrypted = Rot13::new("Hello World!");

    assert_eq!(
        String::from("Hello World!"),
        encrypted.solve()        
    )
}