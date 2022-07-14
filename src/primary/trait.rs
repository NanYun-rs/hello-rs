// trait 特征定义了一个可以被共享的行为，只要实现了特征，你就能使用该行为。
// 对代码逻辑进行抽象 类似 interfa

use std::fmt::Display;

// 调用方法引入特征
use std::convert::TryInto;

pub trait Summary {
    fn summarize(&self) -> String {
        // 可以默认实现、或者不实现
        String::from("default")
    }
}

pub struct Post {
    pub title: String,
}

pub struct Weibo {
    pub user: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        // 方法重载
        format!("{}", self.title)
    }
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        // 方法重载
        format!("{}", self.user)
    }
}

// 使用 trait 作为 method 参数
pub fn notice(item: &impl Summary) {
    println!("trait as args {}", item.summarize())
}

// 特征约束
pub fn notice_1<T: Summary>(item: &T, item1: &T) {
    println!("trait as args {}", item.summarize(), item1.summarize())
}

// 多重约束
pub fn notice_2<T: Summary + Display>(item: &T) {}
pub fn notice_3(item: &(Summary + Display)) {}

// where 约束

pub fn notice_4<T, U>(x: &T, y: &U)
where
    T: Summary + Display,
    U: Display,
{
    ()
}

// derive 派生特征
// 被 derive 标记的对象会自动实现对应的默认特征代码，继承相应的功能。
// 例如 Debug 特征，它有一套自动实现的默认代码，当你给一个结构体标记后，就可以使用 println!("{:?}", s) 的形式打印该结构体的对象。
// #[derive(Debug)]

fn main() {
    let t1 = Post {
        title: String::from("rust"),
    };
    let t2 = Weibo {
        user: String::from("rs"),
    };
    println!("t1 {}", t1.summarize());
    println!("t2 {}", t2.summarize());

    // 调用方法引入特征
    let a: i32 = 10;
    let b: u16 = 20;
    // 代码中引入了 std::convert::TryInto 特征，但是却没有使用它，可能有些同学会为此困惑，主要原因在于如果你要使用一个特征的方法，那么你需要引入该特征到当前的作用域中，我们在上面用到了 try_into 方法，因此需要引入对应的特征。
    let b1 = b.try_into().unwrap();

    if a < b1 {
        println!(" a is less then b")
    }
}
