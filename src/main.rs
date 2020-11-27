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
        let name = "Peter";
        println!("My name is {}.", name);
        println!("{0}'s name is {0}.", name);
        // 调用私有方法
        say_hello();
    }
}

// 引入外部模块
mod mod_variable;
mod mod_contition;
mod mod_function;

fn main() {
    // 注释1
    /* 注释2 */
    use hello::hello;
    hello();

    mod_variable::helloDefine();
    mod_variable::helloDataType();
    mod_contition::helloContition();
    mod_contition::helloMatch();
    mod_function::helloFn();
    mod_function::helloLambda();
}