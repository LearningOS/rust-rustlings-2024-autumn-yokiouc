mod sausage_factory {
    // 让外部模块可以访问 make_sausage
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }

    // 这个函数保持私有，因为不希望外部模块访问
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }
}

fn main() {
    sausage_factory::make_sausage();
}

