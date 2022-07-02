struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 关联函数
    // 定义在 impl 中且没有 self 的函数被称之为关联函数
    // 调用时使用 Rectangle::new
    fn new(w: u32, h: u32) -> Rectangle {
        Rectangle {
            width: w,
            height: h,
        }
    }
    // 现实有self 叫方法 调用函数使用 '.' 运算符
    // &self 替代 rectangle: &Rectangle，&self 其实是 self: &Self 的简写
    // self 具有所有权概念
    // self | &self | &mut self
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // 方法名跟结构体字段名相同
    // a.width | a.width()
    // fn width(&self) -> () {}
}

// struct 可以使用多个 impl 块
impl Rectangle {
    // 带有参数的方法
    fn third_part(&self, third: u32) -> u32 {
        self.width + self.height + third
    }
}

enum Class {
    Math,
    Science,
}

impl Class {
    fn get_class(&self) {
        println!("enum methods")
    }
}
fn main() {
    let a = Rectangle {
        width: 10,
        height: 20,
    };
    let a1 = a.area();
    // 方法
    let a2 = a.third_part(66);
    // 关联函数
    let _a3 = Rectangle::new(10, 20);
    println!("a.area is {}", a1);
    println!("a2.third_part is {}", a2);
    // 为 enums 实现 method 
    let _b1 = Class::Math.get_class();
    let _b2 = Class::Science.get_class();
    // println!("Class has {}", b1)
}
