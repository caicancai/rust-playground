fn main(){
    fn function (i: i32) -> i32 { i + 1 }

    let closure_annotated = |i: i32| -> i32 { i + 1};
    let closure_inferred  = |i     |          i + 1;

    let i = 1;

    // 调用函数和闭包。
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));
    
    // 没有参数的闭包，返回一个 `i32` 类型。
    // 返回类型是自动推导的。
    let one = || 1;
    println!("closure returning one: {}", one());
}