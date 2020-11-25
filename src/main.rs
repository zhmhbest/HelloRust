#![allow(non_snake_case)]

fn helloPrint() {
    // 不换行输出
    print!("in-line");
    // 输出{}
    println!("{{}}");
    // 格式化输出
    let name = "Peter";
    println!("My name is {}.", name);
    println!("{0}'s name is {0}.", name);
}

fn helloVariable() {
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

fn helloDataType() {
    let _ : &str;
    let _ : i8;
    let _ : u8;
    let _ : i16;
    let _ : u16;
    let _ : i32;
    let _ : u32;
    let _ : i64;
    let _ : u64;
    let _ : i128;
    let _ : u128;
    let _ : isize;
    let _ : usize;
    let _ : f32;
    let _ : f64;
    let _ : bool; // true | false
    let _ : char;
    // 元组
    let _tup: (i32, f32, u8) = (500, 6.4, 1);
    // 数组（必须同类型）
    let _arr = [1, 2, 3, 4, 5];
}


fn helloFunction(name: &str) {
    println!("Funtion {}", name);
    // 
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

fn main() {
    // 注释1
    /* 注释2 */
    println!("Hello!");
    helloPrint();
    helloVariable();
    helloDataType();
    helloFunction("fn")
}