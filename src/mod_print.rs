pub fn helloPrint() {
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
}