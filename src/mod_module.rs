fn say_hello() {
    println!("Hello mod_module!");
}

// 自定义模块
mod demo_module {
    // private
    fn say_hello() {
        println!("Hello demo_module!");
    }
    // public
    pub fn hello() {
        // 调用私有方法
        self::say_hello();
        
        // 调用包含当前模块的模块内方法
        super::say_hello();
    }
}

pub fn main() {
    demo_module::hello();
}
