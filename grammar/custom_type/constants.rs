static LANGUAGE: &'static str = "Rust";
const  THRESHOLD: i32 = 10;


//const: 不可改变的值
//static:具有static生命周期的，可以是可变的变量
fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main(){
    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
}