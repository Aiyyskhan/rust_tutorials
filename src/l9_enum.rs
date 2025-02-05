#[derive(Debug)]
enum IpAddKind {
    V4,
    V6,
}

pub fn exmpl_1() {
    let ip_v4 = IpAddKind::V4;
    let ip_v6 = IpAddKind::V6;

    println!("{ip_v4:?}, {ip_v6:?}");
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

pub fn exmpl_2() {
    let ip_v4 = IpAddr::V4(String::from("127.0.0.1"));
    let ip_v6 = IpAddr::V6(String::from("::1"));

    // println!("{ip_v4:?}, {ip_v6:?}");

    match ip_v4 {
        IpAddr::V4(addr) => println!("IPv4: {addr}"),
        IpAddr::V6(addr) => println!("IPv6: {addr}"),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Color(u8, u8, u8),
}

impl Message {
    fn describe(&self) -> String {
        match self {
            &Message::Quit => String::from("quit"),
            Self::Move { x, y } => format!("Move to ({x}, {y})"),
            Self::Write(msg) => format!("Message: {msg}"),
            Self::Color(r, g, b) => format!("Color: ({r}, {g}, {b})"),
        }
    }
}

pub fn exmpl_3() {
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Привет, Rust!"));
    let msg4 = Message::Color(255, 165, 0);

    println!("{}", msg1.describe());
    println!("{}", msg2.describe());
    println!("{}", msg3.describe());
    println!("{}", msg4.describe());
}

pub fn exmpl_4() {
    // Проверка наличия значения с match
    let x: Option<i8> = Some(100); //None; //Some(10);
    match x {
        Some(value) => println!("Значение: {value}"),
        None => println!("Нет значения"),
    }

    // is_some() и is_none(): Проверяют, содержит ли Option значение или является пустым.
    let mut x: Option<i32> = Some(3);
    println!("Есть значение? {}", x.is_some()); // true
    println!("Пусто? {}", x.is_none());         // false

    // unwrap(): Возвращает значение, если оно есть. Если значение отсутствует (None), программа завершится с ошибкой.
    x = Some(42);
    println!("{}", x.unwrap()); // 42

    // unwrap_or(default): Возвращает значение, если оно есть, или переданное значение по умолчанию.
    x = None;
    println!("{}", x.unwrap_or(0)); // 0

    // unwrap_or_else(): Позволяет передать замыкание для вычисления значения по умолчанию.
    x = None;
    let k = 50;
    let default = || k * 100;
    println!("{}", x.unwrap_or_else(default)); // 100

    // map(): Применяет функцию к значению внутри Option, если оно есть.
    x = Some(5);
    let y = x.map(|v| v * 2); // Умножаем значение на 2
    println!("{:?}", y); // Some(10)

    // and_then(): Применяет функцию, возвращающую Option, для обработки текущего значения.
    x = Some(2);
    let y = x.and_then(|v| if v > 3 { Some(v * 2) } else { None });
    println!("{:?}", y); // Some(10)
}