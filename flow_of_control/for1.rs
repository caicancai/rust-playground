/**
 * for in 结构可以遍历一个 Iterator（迭代器）。创建迭代器的一个最简单的方法是使用区间标记 a..b。这会生成从 a（包含此值） 到 b（不含此值）的，步长为 1 的一系列值
 或者，可以使用a..=b表示两端都包含在内的范围
 */
fn main() {
    // `n` 将在每次迭代中分别取 1, 2, ..., 100
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
