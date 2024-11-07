#[macro_use] // 通过 #[macro_use] 导入模块中的宏
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!(); // 现在可以使用宏了
}

