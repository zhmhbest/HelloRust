fn demo_vector() {
    let mut vec_u8: Vec<u8> = Vec::new();
    vec_u8.push(77u8);
    vec_u8.push(99u8);
    println!("vec_u8 = {:?}", vec_u8);
    for i in &vec_u8 {
        println!("{}", i);
    }

    // 容器自动推断
    let mut vec_auto = Vec::new();
    vec_auto.push(66u8);
    vec_auto.push(88u8);
    for i in &mut vec_auto {
        *i -= 2;
    }
    println!("vec_auto = {:?}", vec_auto);
}

fn demo_map() {
    use std::collections::HashMap;
    let mut kv1 = HashMap::new();
    kv1.insert("Blue", 10);
    kv1.entry("Yellow").or_insert(50);
    kv1.entry("Blue").or_insert(50);
    println!("{:?}", kv1);

    let mut kv2 : HashMap<i32, String> = HashMap::new();
    kv2.insert(0, String::from("A"));
    kv2.insert(1, String::from("B"));
    kv2.insert(2, String::from("C"));
    println!("{:?}", kv2);
}

pub fn main() {
    demo_vector();
    demo_map();
}
