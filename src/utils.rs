pub fn text_preprocessor(text: &str) -> String {
    let mut result = String::new();
    for c in text.to_lowercase().chars() {
        if c != ' ' {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod utils_tests {

    use super::*;

    #[test]
    fn test_remove_whitespace() {
        assert_eq!("helloworld!", text_preprocessor("Hello World!"));
    }
}
