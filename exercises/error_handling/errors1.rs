pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // 返回一个错误信息，而不是 None
        Err("`name` was empty; it must be nonempty.".into())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // 不要改变这行
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}

