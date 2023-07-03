mod historical;

use historical::{Caesarcipher, Solve, Rot13, ToString};

//use crate::historical::SolveCaesarCipher;
fn main() {
    let caesarciphertest = Caesarcipher::new("Hello World!", 3).to_string();
    println!("{caesarciphertest}");
    
    let foo = Caesarcipher::decipher(caesarciphertest.as_str(), 3);
    println!("{foo}");






}
