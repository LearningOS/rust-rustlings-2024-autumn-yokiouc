fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");                                 // &str
    string("red".to_string());                            // String
    string(String::from("hi"));                           // String
    string("rust is fun!".to_owned());                    // String
    string("nice weather".into());                        // String
    string(format!("Interpolation {}", "Station"));       // String
    string_slice(&String::from("abc")[0..1]);             // &str (slice of String)
    string_slice("  hello there ".trim());                // &str (trim returns &str)
    string("Happy Monday!".to_string().replace("Mon", "Tues"));  // String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());      // String (to_lowercase returns String)
}

