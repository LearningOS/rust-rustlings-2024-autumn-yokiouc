struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!");
        }
        Rectangle { width, height }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // 检查矩形的宽度和高度是否正确
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10);   // 检查宽度
        assert_eq!(rect.height, 20);  // 检查高度
    }

    #[test]
    #[should_panic(expected = "Rectangle width and height cannot be negative!")]
    fn negative_width() {
        // 检查当宽度为负数时，是否会触发 panic
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic(expected = "Rectangle width and height cannot be negative!")]
    fn negative_height() {
        // 检查当高度为负数时，是否会触发 panic
        let _rect = Rectangle::new(10, -10);
    }
}

