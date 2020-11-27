#![allow(non_snake_case)]

// 自定义模块
mod hello {
    // private
    fn say_hello() {
        println!("Hello!");
    }
    // public
    pub fn hello() {
        // 不换行输出
        print!("in-line");
        // 输出{}
        println!("{{}}");
        // 格式化输出
        // - ``, which uses the `Display` trait
        // - `?`, which uses the `Debug` trait
        // - `e`, which uses the `LowerExp` trait
        // - `E`, which uses the `UpperExp` trait
        // - `o`, which uses the `Octal` trait
        // - `p`, which uses the `Pointer` trait
        // - `b`, which uses the `Binary` trait
        // - `x`, which uses the `LowerHex` trait
        // - `X`, which uses the `UpperHex` trait
        let name = "Peter";
        let age = 16;
        println!("My name is {}.", name);
        println!("{0}'s name is {0}.", name);
        println!("{yourname}'s name is {yourname}.", yourname=name);
        println!("I'm 0b{:b} years old.", age);
        println!("I'm 0x{:x} years old.", age);
        // 调用私有方法
        say_hello();
    }
}

// 引入外部模块
mod mod_variable;
mod mod_contition;
mod mod_function;

fn main() {
    // // Line comments which go to the end of the line.
    // /* Block comments which go to the closing delimiter. */
    // /// Generate library docs for the following item.
    // //! Generate library docs for the enclosing item.
    use hello::hello;
    hello();
    mod_variable::helloDefine();
    mod_variable::helloDataType();
    mod_contition::helloContition();
    mod_contition::helloMatch();
    mod_function::helloFn();
    mod_function::helloLambda();
}