fn all_caps(word: &str) -> String {
    word.to_uppercase()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn check_all_caps() {
        let result = all_caps("hello");
        let expected = String::from("HELLO");
        assert_eq!(result , expected , "String should be in uppercase"); 
    }
}

