fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    // Step 1: 使用 iter() 方法创建一个迭代器
    let mut my_iterable_fav_fruits = my_fav_fruits.iter();   

    // Step 2: 使用 next() 获取下一个元素
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    
    // Step 4: 没有下一个元素时，next() 会返回 None
    assert_eq!(my_iterable_fav_fruits.next(), None);
}

