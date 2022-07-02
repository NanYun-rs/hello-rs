#[derive(Debug)]
struct Pointer {
    x: i32,
    y: i32,
}

fn main() {
    let a = add(Some(3));
    println!("a is {:?}", a);

    let b = 1;

    match b {
        // 单分支多模式
        1 | 2 => println!("1 or 2"),
        // 通过序列 ..= 匹配值的范围
        3..=5 => println!("3 to 5"),
        // 序列只允许用于数字或字符类型，原因是：它们可以连续
        // 同时编译器在编译期可以检查该序列是否为空，字符和数字值是 Rust 中仅有的可以用于判断是否为空的类型。
        // 'a'..'z' => println!("a to z"),
        _ => println!("default"),
    }

    // 解构结构体
    let c = Pointer { x: 1, y: 2 };
    let Pointer { x, y } = c;
    let Pointer { x: x1, y: y1 } = c;
    c_add(x, y);
    println!("x,y,{},{}", x, y);
    println!("pointer x,y,{},{}", x, y);
    println!("pointer x1,y1,{},{}", x1, y1);
}

fn add(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}

fn c_add(x: i32, y: i32) -> i32 {
    x + y
}
