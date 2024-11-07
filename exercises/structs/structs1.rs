// 定义 Classic C 风格的结构体
struct ColorClassicStruct {
    red: u8,
    green: u8,
    blue: u8,
}

// 定义元组结构体
struct ColorTupleStruct(u8, u8, u8);

#[derive(Debug)]
struct UnitLikeStruct; // 定义类单元结构体

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // 实例化一个 Classic C 风格的结构体
        let green = ColorClassicStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // 实例化一个元组结构体
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // 实例化类单元结构体
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}

