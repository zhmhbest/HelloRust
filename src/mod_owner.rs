pub fn main() {
    // 使用并拿走所有权
    fn get_owner(s: String) {
        println!("s1@sub  {}", s);
    }
    let s1 = String::from("Hello");
    println!("s1@main {}", s1);
    get_owner(s1);
    // println!("s@main {}", s1); // value borrowed here after move
    println!("s1@main null");

    // 使用但不拿走所有权
    fn use_reference(s: &String) {
        println!("s2@sub  {}", s);
    }
    let s2 = String::from("Hello");
    println!("s2@main {}", s2);
    use_reference(&s2);
    println!("s2@main {}", s2);
}