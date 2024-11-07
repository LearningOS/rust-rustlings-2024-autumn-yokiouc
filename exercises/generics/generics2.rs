// 使用泛型 T 来替换具体的 u32 类型
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    // 使用泛型 T 让 new 方法接受任何类型的值
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}

