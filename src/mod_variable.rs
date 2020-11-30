fn demo_data_type() {
    let _: i8;
    let _: u8 = b'A';
    let _: u8 = 0b1111_0000; // 下划线起注释作用
    let _: i16;
    let _: u16;
    let _: i32;
    let _: u32;
    let _: i64 = 98_222; // 下划线起注释作用
    let _: u64;
    let _: i128;
    let _: u128;
    let _: isize; // 取决于操作系统位数
    let _: usize; // 取决于操作系统位数
    let _: f32;
    let _: f64;
    let _: char;
    let _: bool; // true | false
}

fn demo_data_struct() {
    // Array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("len = {}", arr.len());
    println!("arr = {:?}", arr);
    let arr: [i32; 10] = [0; 10]; // 10个0
    println!("len = {}", arr.len());
    println!("arr = {:?}", arr);
    // Tuple
    let tup: (i32, f32, u8) = (500, 6.4, 1);
    println!("tup = {:?}", tup);
    // Struct
    #[derive(Debug)]
    struct Student {
        id: i64,
        name: String,
        age: i32,
        gender: bool,
    }
    let student = Student {
        id: 9955845621,
        name: String::from("Peter"),
        age: 0x1a,
        gender: false,
    };
    println!("student = {:?}", student);

    // Tuple Struct
    #[derive(Debug)]
    struct Color(u8, u8, u8);
    let black = Color(0, 0, 0);
    println!("black = {:?}", black);

    // Enums(C-like)
    enum Fruit {
        Apple,
        Banana = 3,
        Watermelon,
    }
    println!("Fruit::Apple is {}", Fruit::Apple as i32);
    println!("Fruit::Banana is {}", Fruit::Banana as i32);
    println!("Fruit::Watermelon is {}", Fruit::Watermelon as i32);

    // Enums
    #[allow(dead_code)]
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 },
    }
}

fn demo_define() {
    // 不可变变量
    let a = 111;
    // a = "abc"; // expected integer, found `&str`
    // a = 1.11;  // expected integer, found floating-point number
    // a = 222;   // cannot assign twice to immutable variable
    println!("a = {} @{:p}", a, &a);
    // 重新绑定不可变变量的值
    let a = 222;
    println!("a = {} @{:p}", a, &a);

    // 可变变量
    let mut b = 123;
    println!("mut b = {} @{:p}", b, &b);
    b = 456; // 改变b的值
    println!("mut b = {} @{:p}", b, &b);

    // 变量类型
    let c1: i32 = 999;
    println!("c1 = {} @{:p}", c1, &c1);
    let c2 = 3.21f32;
    println!("c2 = {} @{:p}", c2, &c2);

    // 常量（常量不可重现绑定）
    const PI: f32 = 3.14;
    // PI = 0.618; // cannot assign to this expression
    // const PI = 0.618; // redefined here
    println!("PI = {}", PI);

    // 重影（指变量的名称可以被重新使用）
    let shadow = 1;
    println!("shadow = {} @{:p}", shadow, &shadow);
    let shadow = "a";
    println!("shadow = {} @{:p}", shadow, &shadow);
}

pub fn main() {
    demo_data_type();
    demo_data_struct();
    demo_define();
}
