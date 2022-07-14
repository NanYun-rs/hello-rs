fn main() {
    // if condition true; else if condition ; else
    // 用 if 来赋值时，要保证每个分支返回的类型一样
    // 可以使用 match 解决 else if 问题
    let index = 1;
    let _i = if index == 1 {
        2
    } else if index == 2 {
        3
    } else {
        10
    };

    // for 循环； for 元素 in 集合
    // for item in  collection - 转移所有权
    let a0 = [1, 2, 3, 4, 5];
    for i in a0 {
        println!("{}", i);
    }
    // println!("a0 is {:?}", a0);
    // for item in  &collection - 不可变借用
    // for item in &mut collection - 可变借用

    // 循环中获取索引
    for (i, v) in a0.iter().enumerate() {
        println!("i={},v={}", i, v)
    }

    // 循环中不声明变量
    for _ in 1..9 {
        println!("i")
    }

    // contiune 跳出循环

    for i in 1..5 {
        if i == 2 {
            continue;
        }
        println!("{}", i);
    }

    // break 退出循环

    for i in 1..5 {
        if i == 2 {
            break;
        }
        println!("{}", i);
    }

    // while 循环
    let mut w = 0;
    while w <= 5 {
        w += 1
    }

    // loop
}
