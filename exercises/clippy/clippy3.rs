#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // 如果 my_option 是 None，不需要 unwrap，直接继续
    if my_option.is_none() {
        // Do nothing
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 创建一个空的 Vec
    let my_empty_vec: Vec<i32> = Vec::new();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 使用标准库的 swap 函数
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}

