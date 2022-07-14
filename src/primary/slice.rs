fn main() {
    let s = String::from("hello world!");
    let f = first_word(&s);
    // s.clear();
    println!("first is {},s is {}", f, s);

    // 字符串字面量 类型是 &str
    // 该切片指向了程序可执行文件中的某个点，这也是为什么字符串字面量是不可变的，因为 &str 是一个不可变引用。
    let s1: &str = "hello &str";
    println!("s1 is {}", s1);

    // 类型转换 &str 转为 String
    let s2 = String::from("form &str to String");
    let s3 = "from &str to String".to_string();
    println!("s2 is {}, s3 is {}", s2, s3);

    // 类型转换 String 转为 &str 取引用

    show_word(&s2);
    show_word(&s2[..2]);
    show_word(s2.as_str());

    // 字符串索引 Rust 不支持通过索引下标访问元素的能力 由于&str String 是 unicode 编码，每个字符占用的长度不固定
    // 一下代码会报错
    // the type `String` cannot be indexed by `{integer}`
    // let si = s2[0];

    // 字符串切片非常危险，中字占三个字节，2没有落在字符的边界上；
    // 因此在通过索引区间来访问字符串时，需要格外的小心，一不注意，就会导致你程序的崩溃！
    // 该行为会导致 panic 程序崩溃
    // let s4 = "中国人";
    // let s5 = &s4[0..2];

    // 操作字符串
    // 字符串必须是 mut
    let mut r = String::from("source string ");

    // 追加 push
    // push 追加字符 char
    // 单引号只能包裹字符char 'p'
    r.push('p');
    println!("after push char  {}", r);
    // push_str() 追加字符串字面量
    // 双引号包裹字符串字面量 “hello”
    r.push_str(" push string");

    println!("after push &str {}", r);

    // insert 插入 （index,value）
    // 第一个参数是字符（串）插入位置的索引，第二个参数是要插入的字符（串），索引从 0 开始计数，如果越界则会发生错误
    let mut r1 = String::from("source string ");
    r1.insert(0, 'i');
    println!("s1 insert {}", r1);
    r1.insert_str(1, " insert string ");
    println!("s1 insert_str {}", r1);

    // replace 替换
    // 返回一个新字符串
    let p = String::from(" rust rust rust!");
    // replace
    // 该方法可适用于 String 和 &str 类型
    // 第一个参数是要被替换的字符串，第二个参数是新的字符串。该方法会替换所有匹配到的字符串。该方法是返回一个新的字符串
    let p1 = p.replace("rust", "RUST");
    // replacen
    // 该方法可适用于 String 和 &str 类型
    // replacen() 方法接收三个参数，前两个参数与 replace() 方法一样，第三个参数则表示替换的个数。该方法是返回一个新的字符串
    let p2 = p.replacen("rust", "RUST", 1);
    //replace_range
    //该方法仅适用于 String 类型。replace_range 接收两个参数，第一个参数是要替换字符串的范围（Range），第二个参数是新的字符串。该方法是直接操作原来的字符串
    //不会返回新的字符串
    let mut mp = String::from(" rust rust rust!");
    mp.replace_range(..6, "R");

    println!("replace,{},replacen{},replace_range,{}", p1, p2, mp);

    // 删除
    // pop —— 删除并返回字符串的最后一个字符
    // 该方法是直接操作原来的字符串。但是存在返回值，其返回值是一个 Option 类型，如果字符串为空，则返回 None

    let mut d1 = String::from("use rust in 中文!");
    let d2 = d1.pop();
    let d3 = d1.pop();
    println!("d1 = {:?} , d2 = {:?}, d3 = {:?}", d1, d2, d3);

    // remove —— 删除并返回字符串中指定位置的字符
    // 方法是直接操作原来的字符串。但是存在返回值，其返回值是删除位置的字符串，只接收一个参数，表示该字符起始索引位置。
    // remove() 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误
    // 删除一个字符串
    let mut d4 = String::from("use rust in 中文!");
    let d5 = d4.remove(2);
    println!("d4 ={}, d5={}", d4, d5);

    // truncate —— 删除字符串中从指定位置开始到结尾的全部字符
    // 该方法是直接操作原来的字符串。无返回值。该方法 truncate() 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误。
    let mut d5 = String::from("use rust in 中文!");
    d5.truncate(3);
    println!("d5 ={}", d5);

    //clear —— 清空字符串
    // 该方法是直接操作原来的字符串。调用后，删除字符串中的所有字符，相当于 truncate() 方法参数为 0 的时候。
    let mut d6 = String::from("use rust in 中文!");
    d6.clear();
    println!("d6 ={}", d6);

    // 连接字符串 连接 (Catenate)
    // 使用 + 或者 += 连接字符串，要求右边的参数必须为字符串的切片引用（Slice)类型。其实当调用 + 的操作符时，相当于调用了 std::string 标准库中的 add() 方法，
    // 这里 add() 方法的第二个参数是一个引用的类型。因此我们在使用 +， 必须传递切片引用类型。不能直接传递 String 类型。+ 和 += 都是返回一个新的字符串。所以变量声明可以不需要 mut 关键字修饰。
    // fn add(self, s: &str) -> String
    // c1 这个变量通过调用 add() 方法后，所有权被转移到 add() 方法里面， add() 方法调用后就被释放了，同时 s1 也被释放了。再使用 s1 就会发生错误。
    let c1 = String::from(" hello");
    let c2 = String::from(" rust");
    let mut c3 = c1 + &c2;
    c3 += " !!!";
    println!("c3 ={}", c3);
    // Panic: borrow of moved value: `c1`
    // println!("c1 is borrowed {}", c1);

    // 使用 format! 连接字符串
    // format! 这种方式适用于 String 和 &str 。format! 的用法与 print! 的用法类似，详见格式化输出。
    let c4 = String::from("hello");
    let c5 = String::from("RUST");
    let c6 = format!("{} {}!!", c4, c5);
    println!("c6={}", c6);

    // 字符串转义
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);

    println!("{}", "hello \\x52\\x75\\x73\\x74");
    // 当然，在某些情况下，可能你会希望保持字符串的原样，不要转义:
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

    // 操作 UTF-8 字符串

    // chars
    for i in "中文".chars() {
        println!("{}", i)
    }

    // bytes
    for i in "中文".bytes() {
        println!("{}", i)
    }
}

fn show_word(s: &str) {
    println!("s is {}", s)
}

fn first_word(s: &String) -> &str {
    &s[..1]
}
