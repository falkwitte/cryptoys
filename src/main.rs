mod historical;

use historical::{Caesarcipher, Solve, Rot13, ToString};

//use crate::historical::SolveCaesarCipher;
fn main() {
    let caesarciphertest = Caesarcipher::new("Hello World!", 3).to_string();
    println!("{caesarciphertest}");
    
    let foo = Caesarcipher::decipher(caesarciphertest.as_str(), 3);
    println!("{foo}");

    let bar = Rot13::new("Hello World!").to_string();
    println!("{bar}");

    let bar2 = Rot13::decipher("Uryyb Jbeyq!");
    println!("{bar2}");



}
