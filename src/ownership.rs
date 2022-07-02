fn main() {
    // String 可变长度类型 存放在堆上
    // String 类型是一个复杂类型，由存储在栈中的堆指针、字符串长度、字符串容量共同组成，
    // 其中堆指针是最重要的，它指向了真实存储字符串内容的堆内存
    // 容量是堆内存分配空间的大小，长度是目前已经使用的大小。
    let mut hello = String::from("hello");
    hello.push_str(", world!");
    println!("say {}", hello);

    // b 只是引用了存储在二进制中的字符串 "hello, world"，并没有持有所有权。
    let a = "hello owndership";
    let b = a;
    println!("a,b,{},{}", a, b);

    let s1 = String::from("s1");
    // 对数据做深拷贝
    let s2 = s1.clone();
    println!("s1,s2,{},{}", s1, s2);

    let m = String::from("ship move");

    takes_ownership(m);
    // 所有权转移到 takes_ownership 无法在后续的代码里面访问
    // println!("move m {}",m);

    let m1 = 123;
    makes_copy(m1);
}

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
}
