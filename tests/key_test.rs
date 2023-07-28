use cryptoys::key::otp;
use cryptoys::Solve;

#[test]
fn otp_encryption_test() {
    let pad: Vec<u8> = vec![1, 3, 3, 7];

    assert_eq!("Obkkh", otp::encrypt(pad, "Hello").to_string())
}

#[test]
fn otp_decryption_test() {
    let pad: Vec<u8> = vec![1, 3, 3, 7];

    assert_eq!("Hello".to_string(), otp::decrypt(pad, "Obkkh"))
}

#[test]
fn otp_solve_trait_test() {
    let pad: Vec<u8> = vec![1, 3, 3, 7];

    assert_eq!("Hello".to_string(), otp::encrypt(pad, "Hello").solve())
}
