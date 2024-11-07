fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) { // 传递字符串的引用
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

