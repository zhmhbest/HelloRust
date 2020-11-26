
pub fn helloPrint() {
    // 不换行输出
    print!("in-line");
    // 输出{}
    println!("{{}}");
    // 格式化输出
    let name = "Peter";
    println!("My name is {}.", name);
    println!("{0}'s name is {0}.", name);
}

pub fn helloVariable() {
    // 不可变变量
    let a = 111;
    // a = "abc"; // expected integer, found `&str`
    // a = 1.11;  // expected integer, found floating-point number
    // a = 222;   // cannot assign twice to immutable variable
    println!("a = {}", a);
    let a = 222;  // 重新绑定不可变量的值
    println!("a = {}", a);

    // 常量（常量不可重现绑定）
    const PI: f32 = 3.14;
    println!("PI = {}", PI);

    // 可变变量
    let mut b = 123;
    println!("b = {}", b);
    b = 456;
    println!("b = {}", b);

    // 重影（指变量的名称可以被重新使用）
    let shadow = 1;
    println!("shadow = {}", shadow);
    let shadow = "a";
    println!("shadow = {}", shadow);
}

pub fn helloContition() {
    if false {
        //
    } else if false {
        //
    } else {
        //
    }
    let r = if true { 1 } else { 0 };
    println!("r = {}", r);
    //
    while false {
        continue;
    }
    //
    loop {
        break;
    }
    //
    let a = [10, 20, 30, 40, 50];
    for i in 0..5 {
        println!("a[{}] = {}", i, a[i]);
    }
}

pub fn helloDataType() {
    let _ : &str;
    let _ : i8;
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
    let _ : bool; // true | false
    let _ : char;
    // 数组（必须同类型）
    let _arr = [1, 2, 3, 4, 5];
    // 元组
    let _tup: (i32, f32, u8) = (500, 6.4, 1);
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
    // 枚举
    // enum Fruit {
    //     Apple(i32),
    //     Banana(i32),
    //     Watermelon(i32),
    // }
}

pub fn helloFunction() {
    let r = {
        let x = 3;
        x + 1
    };
    println!("r = {}", r);
    //
    fn five() -> i32 {
        5
    }
    println!("five() = {}", five());
    //
    fn add(a: i32, b: i32) -> i32 {
        return a + b;
    }
    println!("add(1, 2) = {}", add(1, 2));
}

pub fn helloOwner() {
    // 每个值都有一个所有者。
    // 一次只能有一个所有者。
    // 当所有者不在程序运行范围时，该值将被删除。

}