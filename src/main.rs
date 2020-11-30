#![allow(non_snake_case)]

// 引入外部模块
// mod mod_comment;
mod mod_contition;
mod mod_function;
mod mod_module;
mod mod_print;
mod mod_variable;
mod mod_owner;
mod mod_collection;

fn main() {
    mod_module::main();
    mod_print::main();
    mod_variable::main();
    mod_owner::main();
    mod_contition::main();
    mod_function::main();
    mod_collection::main();
}
