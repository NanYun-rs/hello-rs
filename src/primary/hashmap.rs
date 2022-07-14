use std::collections::HashMap;
fn main() {
    // 所有的集合类型都是动态的，意味着它们没有固定的内存大小，
    // 因此它们底层的数据都存储在内存堆上，然后通过一个存储在栈中的引用类型来访问。同时，跟其它集合类型一致，HashMap 也是内聚性的，即所有的 K 必须拥有同样的类型，V 也是如此。
    let mut map = HashMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);

    // vec 转成 hashmap

    let v1 = vec![(String::from("china"), 100), (String::from("usa"), 10)];
    // 先转成迭代器 在转成 集合 hashmap
    // HashMap<_, _> 代表需要 rust 编译器帮助推导出具体的类型
    // collect 方法在内部实际上支持生成多种类型的目标集合，因为我们需要通过类型标注 HashMap<_,_>
    // 来告诉编译器：请帮我们收集为 HashMap 集合类型，具体的 KV 类型，麻烦编译器您老人家帮我们推导。
    let mut map1: HashMap<_, _> = v1.into_iter().collect();

    println!("hashmap map1 is {:?}", map1);

    // 获取数据 get 方法返回一个 option

    let map_v1: Option<&i32> = map1.get(&String::from("china"));
    println!("map_v1 is {:?}", map_v1);

    // 循环遍历

    for (key, value) in &map1 {
        println!("key is {:?},value is {:?}", key, value);
    }

    // 数据更新

    // 1. 覆盖已有的值

    map1.insert(String::from("china"), 200);
    println!("current value is {:?}", map1.get(&String::from("china")));

    // 2. 查询已有的值、若不存在则插入数据

    map1.entry(String::from("kor")).or_insert(10);

    println!("current value is {:?}", map1.get(&String::from("kor")))
}
