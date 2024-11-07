use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 获取当前的时间戳
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let timestamp = since_the_epoch.as_secs();

    // 输出符合 Cargo 格式的环境变量设置（用于 tests7）
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 检查是否启用了 `pass` 功能标志（用于 tests8）
    if std::env::var("CARGO_FEATURE_PASS").is_ok() {
        println!("cargo:rustc-cfg=feature=\"pass\"");
    }
}

