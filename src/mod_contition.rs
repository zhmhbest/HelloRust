fn demo_if() {
    if false {
    } else if false {
    } else {
    }
    // 三元表达式
    let r = if true { 1 } else { 0 };
    println!("r = {}", r);
}

fn demo_loop() {
    while false {
        continue;
    }
    // 无限循环
    loop {
        break;
    }
    // 遍历范围 [1, 3)
    for i in 1..3 {
        println!("{}", i);
    }
    // 遍历范围 [1, 3]
    for i in 1..=3 {
        println!("{}", i);
    }
    // 遍历列表
    let a = [10, 20, 30, 40, 50];
    for i in 0..5 {
        println!("a[{}] = {}", i, a[i]);
    }
    // 遍历对象列表
    #[derive(Debug)]
    struct Color {
        r: u8,
        g: u8,
        b: u8,
    }
    let arr = [
        Color {
            r: 128,
            g: 255,
            b: 90,
        },
        Color { r: 0, g: 3, b: 254 },
        Color { r: 0, g: 0, b: 0 },
    ];
    for color in arr.iter() {
        println!("{:?}", color);
    }
}

fn demo_match() {
    fn get_number_name(num: i32) {
        print!("{}, ", num);
        match num {
            1 => println!("One"),
            2 | 3 => println!("Two | Three"),
            4..=7 => println!("Four ~ Seven"),
            _ => {
                println!("Unknow");
            }
        }
    }
    for i in 1..=8 {
        get_number_name(i);
    }

    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    fn value_in_cents(coin: Coin) -> u8 {
        print!("Coin({:?}) = ", coin);
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Nickel));
    println!("{}", value_in_cents(Coin::Dime));
    println!("{}", value_in_cents(Coin::Quarter));

    #[derive(Debug)]
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 },
    }
    fn inspect(event: WebEvent) {
        print!("event({:?}) = ", event);
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebEvent::Paste(s) => println!("pasted \"{}\".", s),
            WebEvent::Click { x, y } => {
                println!("clicked at x={}, y={}.", x, y);
            }
        }
    }
    inspect(WebEvent::PageLoad);
    inspect(WebEvent::KeyPress('x'));
    inspect(WebEvent::Paste("my text".to_owned()));
    inspect(WebEvent::Click { x: 20, y: 80 });
    inspect(WebEvent::PageUnload);
}

fn demo_let() {
    let optional = Some(7);
    match optional {
        Some(i) => {
            println!("`{:?}`", i);
        },
        _ => {},
    };
    if let Some(i) = optional {
        println!("If matched `{:?}`!", i);
    }
    while let Some(i) = optional {
        println!("While matched `{:?}`!", i);
        break;
    }

    #[allow(dead_code)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    fn is_penny(coin : Coin) {
        if let Coin::Penny = coin {
            println!("A Penny!");
        }
    }
    is_penny(Coin::Penny);
}

pub fn main() {
    demo_if();
    demo_loop();
    demo_match();
    demo_let();
}
