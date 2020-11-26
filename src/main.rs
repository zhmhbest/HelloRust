#![allow(non_snake_case)]

// 自定义模块
mod hello {
    // private
    fn get_hello() -> String {
        return String::from("Hello!");
    }
    // public
    pub fn hello() {
        println!("{}", get_hello());
    }
}

mod base; // 引入外部模块

fn main() {
    // 注释1
    /* 注释2 */
    use hello::hello;
    hello();

    base::helloPrint();
    base::helloVariable();
    base::helloContition();
    base::helloDataType();
    base::helloFunction();
    base::helloOwner();
}