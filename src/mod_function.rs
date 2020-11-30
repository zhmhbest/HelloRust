fn demo_simply() {
    // 返回最后一个表达式的结果
    let r = {
        let x = 3;
        x + 1
    };
    println!("r = {}", r);
    // 一般函数
    fn add(a: i32, b: i32) -> i32 {
        return a + b;
    }
    println!("add(1, 2) = {}", add(1, 2));
    // 省略return
    fn mul(a: i32, b: i32) -> i32 {
        a * b
    }
    println!("mul(2, 3) = {}", mul(2, 3));
}

fn demo_lambda() {
    // lambda(完整)
    let ladd1 = |x: i32, y: i32| -> i32 { x + y };
    println!("ladd1(2, 3) = {}", ladd1(2, 3));
    // lambda(省略返回类型)
    let ladd2 = |x: i32, y: i32| x + y;
    println!("ladd2(2, 3) = {}", ladd2(2, 3));
    // lambda(省略变量类型、返回类型)
    let ladd3 = |x, y| x + y;
    println!("ladd3(2, 3) = {}", ladd3(2, 3));
}

fn demo_method() {
    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
    }
    impl Point {
        fn new(x: f64, y: f64) -> Point {
            Point { x: x, y: y }
        }
    }
    let point = Point::new(32f64, 64f64);
    println!("{:?}", point);
}

pub fn main() {
    demo_simply();
    demo_lambda();
    demo_method();
}
