mod ownership;

fn animals() {
    let dog = "dog";
    let cat = "cat";
    let mouse = "mouse";
    let animals = [dog, cat, mouse];
    for animal in animals {
        println!("{}", animal)
    }
}

fn main() {
    animals();
    // println!("Hello, world!");
}
