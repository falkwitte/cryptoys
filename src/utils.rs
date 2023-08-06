use std::collections::HashMap;

pub fn text_preprocessor(text: &str) -> String {
    let mut result = String::new();
    for c in text.to_lowercase().chars() {
        if c != ' '{
            result.push(c);
        }
    }
    result
}

pub fn find_key_for_value<'a, T, U>(map: &'a HashMap<U, T>, value: T) -> U
where
    T: PartialEq,
    U: PartialEq + Copy,
{
    *(map.iter().find(|val| val.1 == &value).unwrap().0)
}

#[cfg(test)]
mod utils_tests {

    use super::*;

    #[test]
    fn test_remove_whitespace() {
        assert_eq!("helloworld!", text_preprocessor("Hello World!"));
    }

    #[test]
    fn test_find_key_for_value() {
        let mut hashmap: HashMap<char, i32> = HashMap::new();
        hashmap.insert('c', 5);
        hashmap.insert('d', 4);
        hashmap.insert('a', 1);

        assert_eq!('c', find_key_for_value(&hashmap, 5))
    }
}
