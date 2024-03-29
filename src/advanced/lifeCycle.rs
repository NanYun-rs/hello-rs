fn main() {
    // 异常 case
    // let r;
    // {
    //     let x = 1;
    //     r = &x;
    // }
    // println!("{}", r)

    let x = 1;
    let r;
    r = &x;
    println!("{}", r);

    let a = "hello";
    let b = "rust";
    let c = life(a, b);
    println!("life is {}", c)
}

// Tips: 生命周期标注并不会改变任何引用的实际作用域
// Tips: 在通过函数签名指定生命周期参数时，我们并没有改变传入引用或者返回引用的真实生命周期，而是告诉编译器当不满足此约束条件时，就拒绝编译通过。


// 编译器不知道 a b 两个变量的生命周期、所以要进行标注
// 和泛型一样，使用生命周期参数，需要先声明 <'a>
// x、y 和返回值至少活得和 'a 一样久(因为返回值要么是 x，要么是 y)
fn life<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

// 函数的返回值如果是一个引用类型，那么它的生命周期只会来源于：

// 函数参数的生命周期
// 函数体中某个新建引用的生命周期

// 生命周期语法用来将函数的多个引用参数和返回值的作用域关联到一起，一旦关联到一起后，Rust 就拥有充分的信息来确保我们的操作是内存安全的。

// 生命周期保证参数和返回值的生命周期是一样的