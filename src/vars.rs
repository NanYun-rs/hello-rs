fn main() {
    mutable();
    un_used();
    destructuring_assignment();
    shadowing();
}

// mutable 可变性与不可变性

fn mutable() {
    // 不可变
    let a = 10;
    // 可变
    let mut b = 20;
    println!("b1 = {}", b);
    b = 30;
    println!("a = {},b = {}", a, b);
}

fn un_used() {
    // 声明未使用变量
    let _z = 100;
}

struct Struct {
    e: i32,
}

// 解构赋值
fn destructuring_assignment() {
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 10 };
    let nums = [a, b, c, d, e];
    for num in nums {
        println!("{}", num);
    }
}

// 变量遮蔽
fn shadowing() {
    // 第二个 let 生成了完全不同的新变量，两个变量只是恰好拥有同样的名称，涉及一次内存对象的再分配 
    //，而 mut 声明的变量，可以修改同一个内存地址上的值，并不会发生内存对象的再分配，性能要更好。
    let x = 10;
    let x = x + 1;

    {
        // 变量遮蔽的用处在于，如果你在某个作用域内无需再使用之前的变量（在被遮蔽后，无法再访问到之前的同名变量），
        // 就可以重复的使用变量名字，而不用绞尽脑汁去想更多的名字。
        let x = x * 2;
        println!("inner x is {}", x);
    }
    println!("outer x is {}", x);
}
