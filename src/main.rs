#![allow(non_snake_case)]

// 自定义模块
mod hello {
    // private
    fn say_hello() {
        println!("Hello!");
    }
    // public
    pub fn hello() {
        // 调用私有方法
        say_hello();
    }
}

// 引入外部模块
mod mod_print;
mod mod_variable;
mod mod_contition;
mod mod_function;

fn main() {
    mod_print::main();
    mod_variable::main();
    // use hello::hello; hello();
    // mod_variable::helloDefine();
    // mod_variable::helloDataType();
    // mod_contition::helloContition();
    // mod_contition::helloMatch();
    // mod_function::helloFn();
    // mod_function::helloLambda();
}