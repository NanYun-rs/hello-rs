// 类型转换

use std::convert::TryInto;
fn main() {
    // as 转换
    let a = 3.1 as i8;
    let b = 100_i8 as i32;
    let c = 'a' as i8; // ‘a’ 会转成 i8 => 97

    println!("{},{},{}", a, b, c);

    // tryinto 转换
    // 在一些场景中，使用 as 关键字会有比较大的限制。
    // 如果你想要在类型转换上拥有完全的控制而不依赖内置的转换，例如处理转换错误，那么可以使用 TryInto
    // try_into 会尝试进行一次转换，并返回一个 Result，此时就可以对其进行相应的错误处理。
    let d: u8 = 100;
    let e: u16 = 200;
    //  unwrap 方法，该方法在发现错误时，会直接调用 panic 导致程序的崩溃退出，在实际项目中，请不要这么使用
    let e1: u8 = e.try_into().unwrap();

    if d < e1 {
        println!("e is larger")
    }

    // 最主要的是 try_into 转换会捕获大类型向小类型转换时导致的溢出错误：

    let a1: i32 = 10000;
    let _a2: u8 = match a1.try_into() {
        Ok(x) => x,
        Err(e) => {
            // 捕获异常 err is "out of range integral type conversion attempted"
            println!("err is {:?}", e.to_string());
            0
        }
    };
}
