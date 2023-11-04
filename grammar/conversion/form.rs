use std::convert::From

//From trait 允许一种类型定义 “怎么根据另一种类型生成自己”，因此它提供了一种类型转换的简单机制。
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self{
        Number {value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
}