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
    println!("size(bool) = {}", std::mem::size_of_val(&true));
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
    struct Point {
        x: f32,
        y: f32,
    }
    let point = Point { x: 10.3, y: 0.4 };
    println!("point = {:?}", point);
    let another_point = Point { y: 6.18, ..point };
    println!("another_point = {:?}", another_point);

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
    // use Fruit::Banana;
    // use Fruit::{Apple, Banana, Watermelon};
    use Fruit::*;
    println!("Fruit::Banana is {}", Banana as i32);
    println!("Fruit::Watermelon is {}", Watermelon as i32);

    // Enums
    #[allow(dead_code)]
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 },
    }
    // 需要使用match匹配
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

    // 延迟绑定
    let delay;
    delay = 666;
    // ready = 999; // cannot assign twice to immutable variable
    println!("delay = {}", delay);

    // 可变变量
    let mut b = 123;
    println!("mut b = {} @{:p}", b, &b);
    b = 456; // 改变b的值
    println!("mut b = {} @{:p}", b, &b);

    // 变量冻结
    let mut freezing = 666;
    println!("freezing = {}", freezing);
    {
        #[allow(unused_variables)]
        let freezing = freezing;
        // freezing = 555; // cannot assign twice to immutable variable
    }
    freezing = 999;
    println!("freezing = {}", freezing);

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

fn demo_cast() {
    const NPI: f64 = -3.1415926535897f64; // Literal
    println!("NPI as f64 = {}", NPI);
    println!("NPI as f32 = {}", NPI as f32);
    println!("NPI as i32 = {}", NPI as i32);
    println!("NPI as u32 = {}", NPI as u32);
}

fn demo_alias() {
    #[allow(non_camel_case_types)]
    type uint64_t = u64;
    #[allow(non_camel_case_types)]
    type uint32_t = u32;
    let x = 0 as uint32_t;
    let y = 0 as uint64_t;
    println!("size(uint32_t) = {}", std::mem::size_of_val(&x));
    println!("size(uint64_t) = {}", std::mem::size_of_val(&y));
}

fn demo_from_into() {
    // 系统定义的From
    println!("{}", String::from("hello"));

    // use std::convert::From;
    #[derive(Debug)]
    struct Number {
        value: i32,
    }
    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }
    // The From trait allows for a type to define how to create itself from another type.
    let num = Number::from(1111);
    println!("num = {:?}", num);
    // If you have implemented the From trait for your type, Into will call it when necessary.
    let num: Number = 618.into();
    println!("num = {:?}", num);

    // TryFrom/TryInto traits are used for fallible conversions, and as such, return Results.
    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);
    use std::convert::{TryFrom, TryInto};
    impl TryFrom<i32> for EvenNumber {
        type Error = ();
        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }
    let r = EvenNumber::try_from(8);
    if r.is_ok() {
        let num = r.unwrap();
        println!("num = {:?}", num);
    }
    let r = EvenNumber::try_from(9);
    if r.is_ok() {
        let num = r.unwrap();
        println!("num = {:?}", num);
    }
    let r: Result<EvenNumber, ()> = 6i32.try_into();
    if r.is_ok() {
        let num = r.unwrap();
        println!("num = {:?}", num);
    }
}

pub fn main() {
    demo_data_type();
    demo_data_struct();
    demo_define();
    demo_cast();
    demo_alias();
    demo_from_into();
}
