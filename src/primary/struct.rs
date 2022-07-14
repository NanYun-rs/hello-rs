struct User {
    name: String,
    phone: u64,
    email: String,
    code: u64,
}

#[derive(Debug)]
struct Student {
    name: String,
    age: u8,
    active: bool,
    class: String,
}

fn main() {
    let mut xiaoming = User {
        name: String::from("xiao ming"),
        phone: 66666666666,
        email: String::from("xiaomingu@58.com"),
        code: 166666,
    };

    xiaoming.email = String::from("xiaoming01@58.com");

    println!(
        "xiaoming:{}{}{}{}",
        xiaoming.name, xiaoming.phone, xiaoming.email, xiaoming.code
    );

    // 结构体创建
    let lili = build_user(String::from("lili"), 12);
    println!("lili:{}", lili.age);

    // struct 更新
    // 结构体更新语法跟赋值语句 = 非常相像
    // ..lili 会发生所有权转移 String 类型的所有权会转移、 u8 bool 会调用copy 不会产生所有权转移造成无法访问的问题

    let lucas = Student {
        name: String::from("lucas"),
        // 解构 struct
        ..lili
    };
    println!("lucas class is {}", lucas.class);

    // todo: panic lili.class 的所有权已经转移到 lucas 故无法访问
    // println!("class wonership has changes {}", lili.class)

    // 元祖结构体
    // 结构体必须要有名称，但是结构体的字段可以没有名称，这种结构体长得很像元组，因此被称为元组结构体
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("black color = {:?}", black);

    // Sturct 如果想要借用数据 &str 需要使用声明周期相关概念

    // 打印 Struct 使用 #[derive(Debug)] 打印 Struct 信息

    println!("lucas is {:#?}", lucas);

    // dbg! 宏
    // dbg! 输出到标准错误输出 stderr，而 println! 输出到标准输出 stdout
    let lucy = build_user(String::from("lucy"), dbg!(1 + 2));
    // println!("")
    // dbg!(&lucy)
}

fn build_user(name: String, age: u8) -> Student {
    // 对象
    Student {
        name,
        age,
        active: true,
        class: String::from("class 10"),
    }
}
