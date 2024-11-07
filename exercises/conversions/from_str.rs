use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// 用于表示解析 Person 时的错误
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    Empty,          // 输入为空
    BadLen,         // 分割后的字段数不正确
    NoName,         // 名字为空
    ParseInt(ParseIntError),  // 解析年龄时出错
}

impl FromStr for Person {
    type Err = ParsePersonError;
    
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        // 1. 处理空输入
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }

        // 2. 按逗号分割字符串
        let parts: Vec<&str> = s.split(',').collect();

        // 3. 检查字段数量是否为 2
        if parts.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }

        let name = parts[0].trim();
        let age_str = parts[1].trim();

        // 4. 检查名字是否为空
        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }

        // 5. 解析年龄
        match age_str.parse::<usize>() {
            Ok(age) => Ok(Person {
                name: name.to_string(),
                age,
            }),
            Err(e) => Err(ParsePersonError::ParseInt(e)),
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }

    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}

