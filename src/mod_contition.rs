pub fn helloContition() {
    if false {
        //
    } else if false {
        //
    } else {
        //
    }
    // 三元表达式
    let r = if true { 1 } else { 0 };
    println!("r = {}", r);
    //
    while false {
        continue;
    }
    //
    loop {
        break;
    }
    //
    let a = [10, 20, 30, 40, 50];
    for i in 0..5 {
        println!("a[{}] = {}", i, a[i]);
    }
}


pub fn helloMatch() {
    fn get_number_name(num: i32) {
        match num {
            1 => println!("One"),
            2 | 3 => println!("Two | Three"),
            4..=9 => println!("Four ~ Nine"),
            _ => {
                println!("Unknow");
            }
        }
    }
    get_number_name(1);
    get_number_name(2);
    get_number_name(3);
    get_number_name(4);
    get_number_name(6);
    get_number_name(9);
    get_number_name(10);

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    fn value_in_cents(coin: Coin) -> u8 {
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

    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 },
    }
    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebEvent::Paste(s) => println!("pasted \"{}\".", s),
            WebEvent::Click { x, y } => {
                println!("clicked at x={}, y={}.", x, y);
            },
        }
    }
    inspect(WebEvent::PageLoad);
    inspect(WebEvent::KeyPress('x'));
    inspect(WebEvent::Paste("my text".to_owned()));
    inspect(WebEvent::Click { x: 20, y: 80 });
    inspect( WebEvent::PageUnload);
}