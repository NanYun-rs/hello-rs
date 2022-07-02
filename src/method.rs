struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self 替代 rectangle: &Rectangle，&self 其实是 self: &Self 的简写
    // self 具有所有权概念
    // self | &self | &mut self
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // 方法名跟结构体字段名相同
    // a.width | a.width()
    // fn width(&self) -> () {}

    // 带有参数的方法
    fn third_part(&self, third: u32) -> u32 {
        self.width + self.height + third
    }



    // 关联函数
    // 定义在 impl 中且没有 self 的函数被称之为关联函数
}

fn main() {
    let a = Rectangle {
        width: 10,
        height: 20,
    };
    let a1 = a.area();
    let a2 = a.third_part(66);
    println!("a.area is {}", a1);
    println!("a2.third_part is {}", a2);
}
