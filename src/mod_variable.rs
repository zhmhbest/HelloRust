pub fn helloDefine() {
    // 不可变变量
    let a = 111;
    // a = "abc"; // expected integer, found `&str`
    // a = 1.11;  // expected integer, found floating-point number
    // a = 222;   // cannot assign twice to immutable variable
    println!("a = {} @ {:p}", a, &a);
    let a = 222;  // 重新绑定不可变量的值
    println!("a = {} @ {:p}", a, &a);


    // 可变变量
    let mut b: i32 = 123;
    println!("b = {} @ {:p}", b, &b);
    b = 456; // 改变b的值
    println!("b = {} @ {:p}", b, &b);


    // 常量（常量不可重现绑定）
    const PI: f32 = 3.14; 
    // PI = 0.618; // cannot assign to this expression
    // const PI = 0.618; // redefined here
    println!("PI = {}", PI);


    // 重影（指变量的名称可以被重新使用）
    let shadow = 1;
    println!("shadow = {} @ {:p}", shadow, &shadow);
    let shadow = "a";
    println!("shadow = {} @ {:p}", shadow, &shadow);
}


pub fn helloDataType() {
    let _ : i8;
    let _ : u8 = b'A';
    let _ : u8 = 0b1111_0000; // 下划线起注释作用
    let _ : i16;
    let _ : u16;
    let _ : i32;
    let _ : u32;
    let _ : i64 = 98_222; // 下划线起注释作用
    let _ : u64;
    let _ : i128;
    let _ : u128;
    let _ : isize;
    let _ : usize;
    let _ : f32;
    let _ : f64;
    let _ : char;
    // 布尔
    let _ : bool; // true | false
    // 字符串
    let _ : &str;
    // 数组（必须同类型）
    let _ = [1, 2, 3, 4, 5];
    // 复合类型
    let _: (i32, f32, u8) = (500, 6.4, 1);
    // 结构体
    struct Student {
        id: i64,
        name: String,
    }
    let stu = Student{id: 10010, name: String::from("Peter")};
    println!("{{ id: {}, name: {} }}", stu.id, stu.name);
    // 元组结构体
    struct Color(u8, u8, u8);
    let black = Color(0, 0, 0);
    println!("{{ Color: {} {} {}}}", black.0, black.1, black.2);
    // 枚举类型(C-like)
    enum Fruit {
        Apple = 1,
        Banana = 2,
        Watermelon = 3,
    }
    println!("Apple is {}", Fruit::Apple as i32);
    println!("Banana is {}", Fruit::Banana as i32);
    println!("Watermelon is {}", Fruit::Watermelon as i32);
}