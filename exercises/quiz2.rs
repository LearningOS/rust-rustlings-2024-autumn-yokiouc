pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // 完成函数签名
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // 创建输出的 Vec<String>
        let mut output: Vec<String> = vec![];

        for (string, command) in input.iter() {
            let transformed_string = match command {
                Command::Uppercase => string.to_uppercase(), // 转换为大写
                Command::Trim => string.trim().to_string(),  // 去除两端空格
                Command::Append(times) => {
                    let mut s = string.clone(); // 复制原字符串
                    for _ in 0..*times {
                        s.push_str("bar"); // 追加 "bar"
                    }
                    s
                }
            };
            output.push(transformed_string);
        }

        output
    }
}

#[cfg(test)]
mod tests {
    // 导入 transformer
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}

