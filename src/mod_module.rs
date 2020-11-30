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

pub fn main() {
    hello::hello();
}
