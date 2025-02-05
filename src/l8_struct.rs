struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn exmpl_1() {
    let mut user_1 = User {
        active: true,
        email: String::from("example@mail.com"),
        username: String::from("username123"),
        sign_in_count: 1,
    };

    let user_1_mail = user_1.email;
    println!("user_1 email: {user_1_mail}");

    user_1.username = String::from("Aiyyskhan");
    let user_1_username = user_1.username;
    println!("user_1 username: {user_1_username}");
}

pub fn exmpl_2() {
    let user_2 = build_user(String::from("example@mail.com"), String::from("Aiyyskhan"));
    let user_2_username = user_2.username;
    let user_2_mail = user_2.email;
    println!("user_2 username: {user_2_username}");
    println!("user_2 email: {user_2_mail}");
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Color(u8, u8, u8);
struct Point(u8, u8, u8);

pub fn exmpl_3() {
    let green = Color(0, 255, 0);
    let origin = Point(0, 1, 2);

    // let r = green.0;
    // let g = green.1;
    // let b = green.2;
    // println!("RGB: ({r}, {g}, {b})");

    println!("RGB: ({}, {}, {})", green.0, green.1, green.2);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn exmpl_4() {
    let scale = 2;
    // dbg!(scale);
    let rect_1 = Rectangle {
        // width: 30,
        width: dbg!(30 * scale),
        height: 50,
    };
    // вызов метода
    // println!("The area of the rectangle is {} square pixels.", area(&rect_1));
    println!("The area of the rectangle is {} square pixels.", rect_1.area());
    // println!("rect_1 is {rect_1:#?}.");
    // dbg!(&rect_1);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

pub fn exmpl_5() {
    // вызов ассоциированной функции
    let rect_1 = Rectangle::square(10);
    println!("The area of the rectangle is {} square pixels.", rect_1.area());
}