#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// 实现默认值
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// 实现 From<&str> for Person
impl From<&str> for Person {
    fn from(s: &str) -> Person {
        // 如果输入为空，返回默认值
        if s.is_empty() {
            return Person::default();
        }

        // 以逗号分割字符串
        let parts: Vec<&str> = s.split(',').collect();

        // 如果分割后的部分不等于2，返回默认值
        if parts.len() != 2 {
            return Person::default();
        }

        let name = parts[0].trim();
        let age_str = parts[1].trim();

        // 如果名字为空，返回默认值
        if name.is_empty() {
            return Person::default();
        }

        // 尝试解析年龄
        if let Ok(age) = age_str.parse::<usize>() {
            Person {
                name: name.to_string(),
                age,
            }
        } else {
            // 如果解析年龄失败，返回默认值
            Person::default()
        }
    }
}

fn main() {
    // 使用 From 实现
    let p1 = Person::from("Mark,20");
    // 使用 Into 进行转换
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }

    #[test]
    fn test_bad_convert() {
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_good_convert() {
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }

    #[test]
    fn test_bad_age() {
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}

