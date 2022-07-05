// 泛型

// 使用泛型参数，有一个先决条件，必需在使用前对其进行声明：
// add<T>
// 类型限制 std::ops::Add<Output = T 不是所有的类型都能进行 ’+‘ 操作
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// 结构体使用泛型
// 提前声明，跟泛型函数定义类似，首先我们在使用泛型参数之前必需要进行声明 Point<T>，接着就可以在结构体的字段类型中使用 T 来替代具体的类型
// x 和 y 是相同的类型
struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

// 枚举使用泛型

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// method 使用泛型
// 使用泛型参数前，依然需要提前声明：impl<T>
// 只有提前声明了，我们才能在Point<T>中使用它，这样 Rust 就知道 Point 的尖括号中的类型是泛型而不是具体类型
// 需要注意的是，这里的 Point<T> 不再是泛型声明，而是一个完整的结构体类型，因为我们定义的结构体就是 Point<T> 而不再是 Point。
impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr)
}

fn main() {
    let a = add(1, 2);
    let a1 = add(1.1, 2.2);
    println!("add T {} {}", a, a1);

    let p = Point { x: 1, y: 2 };
    let p1 = Point { x: 1.1, y: 2.1 };
    let px = p.get_x();
    println!("px is {}", px);

    // const 泛型
    // [u8;1] [u8;2] 数据类型并不一致；
    // 使用 const 泛型 可以减少代码编写
    let c = display_array([1, 2, 3, 4]);
    let c1 = display_array([1, 1, 1, 1, 1]);
}
