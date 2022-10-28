fn main() {
    // 闭包 捕获调用者作用域中的值
    // let x = 1;
    // let sum = |y| x + y;
    // let y = sum(10);
    // println!("{}", y);

    // fn_once(|x| x);
    // fn_once(|x| x)

    fn factory() -> Box<dyn Fn(i32) -> i32> {
        let num = 5;
        Box::new(move |x| x + num)
    }

    let f = factory();

    let answer = f(1);
    assert_eq!(6, answer)
}

// fn fn_once<F>(fun: F)
// where
//     F: FnOnce(usize) -> usize,
// {
//     println!("{}", fun(3));
//     println!("{}", fun(4));
//     // fun(4);
//     println!("1")
// }
