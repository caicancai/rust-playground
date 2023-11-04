#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

//单元结构体
struct Unit;

//元组结构体
struct Pair(i32, f32);

//带有两个字段的结构体
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    // 可以在空间中给定左上角和右下角在空间中的位置来指定矩形。
    top_left: Point,
    bottom_right: Point,
}

fn main(){
    let name    = String::from("Peter");
    let age     = 27;
    let peter   = Person{name, age};

    println!("{:?}", peter);

    //实例化结构体‘Point’
    let point: Point = Point {x: 10.3, y:0.4};

    // 访问 point 的字段
    println!("point coordinates: ({}, {})", point.x, point.y);

        // 使用结构体更新语法创建新的 point，
    // 这样可以用到之前的 point 的字段
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` 与 `point.y` 一样，因为这个字段就是从 `point` 中来的
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point { x: left_adge, y:top_edge} = point;
    // let _rectangle = Rectangle {
    //     // 结构体的实例化也是一个表达式
    //     top_left: Point { x: left_edge, y: top_edge },
    //     bottom_right: bottom_right,
    // };
    
    // 实例化一个单元结构体
    let _unit = Unit;

    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);

    // 访问元组结构体的字段
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
    // 实例化一个单元结构体
    let _unit = Unit;

    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);

    // 访问元组结构体的字段
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

}