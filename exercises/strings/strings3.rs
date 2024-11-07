fn trim_me(input: &str) -> String {
    input.trim().to_string() // 使用 trim() 移除字符串两端的空格
}

fn compose_me(input: &str) -> String {
    format!("{} world!", input) // 使用 format! 将 " world!" 连接到字符串的后面
}

fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons") // 使用 replace() 将 "cars" 替换为 "balloons"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}

