pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(2));  // 调用 is_even(2) 并断言结果为 true
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(5));  // 调用 is_even(5) 并断言结果为 false
    }
}

