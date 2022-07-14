#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}
enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn main() {
    let dir = Direction::East;

    // 模式匹配 match
    // match 的匹配必须要穷举出所有可能，因此这里用 _ 来代表未列出的所有可能性
    // match 的每一个分支都必须是一个表达式，且所有分支的表达式最终返回值的类型必须相同
    // X | Y，类似逻辑运算符 或，代表该分支可以匹配 X 也可以匹配 Y，只要满足一个即可
    // match 本身也是一个表达式，因此可以用它来赋值：

    let c_dir = match dir {
        Direction::East => "East",
        Direction::North | Direction::South => "North or South",
        // 穷尽匹配
        // match 的匹配必须穷尽所有情况 否则会抛出 panic
        _ => "West",
    };

    println!("c_dir = {}", c_dir);

    // 模式绑定
    // 可以在match中获取枚举的值
    // 模式匹配的另外一个重要功能是从模式中取出绑定的值

    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("point ({}, {})", x, y);
            }
            Action::ChangeColorRGB(r, g, _) => {
                println!("color into '(r:{}, g:{}, b:0)'", r, g,);
            }
        }
    }

    // if let 匹配
    // 当你只要匹配一个条件，且忽略其他条件时就用 if let ，否则都用 match

    let a = Some(3_u8); // _u8 代表数据类型

    if let Some(3) = a {
        println!("yes some(3)")
    }

    // matches! 宏

    let v = vec![
        Direction::West,
        Direction::East,
        Direction::South,
        Direction::North,
    ];

    // 对 v 进行数据过滤

    // 错误代码示例
    // 因为无法将 x 直接跟一个枚举成员进行比较
    // v.iter().filter(x | x == Direction.West)

    // 正确示例
    // matches! 它可以将一个表达式跟模式进行匹配，然后返回匹配的结果 true or false
    // TODO: 结果不符合预期
    let v1 = v.iter().filter(|x| matches!(x, Direction::West));

    println!("f is {:?}", v1)
}
