use std::collections::HashMap;
fn main() {
    // let a = [1, 2, 3];

    // for i in a {
    //     println!("{}", i)
    // }

    // for b in 1..10 {
    //     println!("{}", b)
    // }

    // let value = vec![1, 2, 3];

    // let _result = match IntoIterator::into_iter(value) {
    //     mut iter => loop {
    //         match iter.next() {
    //             Some(x) => println!("{}", x),
    //             None => break,
    //         }
    //     },
    // };

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);

    // collect 消费者适配器 使用它可以将一个迭代器中的元素收集到指定类型中

    let name = ["lili", "leilei"];
    let age = [12, 13];
    let folks: HashMap<_, _> = name.iter().zip(age.iter()).collect();
    println!("{:?}", folks)
}
