use std::error;
use std::fmt;
use std::num::ParseIntError;

// 使用 Box<dyn error::Error> 作为错误类型
fn main() -> Result<(), Box<dyn error::Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?; // 可能会返回 ParseIntError
    println!("output={:?}", PositiveNonzeroInteger::new(x)?); // 可能会返回 CreationError
    Ok(())
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// 让 CreationError 实现 fmt::Display，这样它可以作为 std::error::Error 的一部分
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

// 让 CreationError 实现 std::error::Error
impl error::Error for CreationError {}

