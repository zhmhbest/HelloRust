pub fn main() {
    // 一行
    print!("line\n");
    println!("line");
    // 输出{}
    println!("{{}}");
    // 输出\
    println!("\\");

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

    #[derive(Debug)] // This make it printable by {:?}.
    struct Student {
        id: i64,
        name: String,
        age: i32,
        gender: bool,
    }
    let student = Student {
        id: 9955845621,
        name: String::from("Peter"),
        age: 0x1a,
        gender: false,
    };
    println!("Hello {}!", student.name);
    println!("{0}'s name is {0}.", student.name);
    println!("{yourname}'s name is {yourname}.", yourname = student.name);

    println!("{0}'s id is {1}.", student.name, student.id);
    println!("{0}'s id is {1:e}.", student.name, student.id);
    println!("{0}'s id is {1:E}.", student.name, student.id);

    println!("{0} is {1} years old.", student.name, student.age);
    println!("{0} is 0b{1:b} years old.", student.name, student.age);
    println!("{0} is 0o{1:o} years old.", student.name, student.age);
    println!("{0} is 0x{1:x} years old.", student.name, student.age);
    println!("{0} is 0X{1:X} years old.", student.name, student.age);

    println!("Student Pointer @{:p}!", &student);
    println!("Student Debug   {:?}!", student);
    // 自定义结构体打印方式
    use std::fmt::{self, Display, Formatter};
    impl Display for Student {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            // return  write!(f, ...);e
            write!(
                f,
                "学生{{学号:{0}, 姓名:{1}, 年龄:{2}, 性别:{3}}}",
                self.id,
                self.name,
                self.age,
                if self.gender { "男" } else { "女" }
            )
        }
    }
    println!("Student Format  {}!", student);
}
