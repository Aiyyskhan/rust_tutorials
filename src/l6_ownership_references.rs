pub fn exmpl_1() {
    let mut s = String::from("Hello");

    s.push_str(", World!");

    println!("{s}");
}

pub fn exmpl_2() {
    let s0 = String::from("string 0");
    let s1 = "string 1";

    // takes_ownership(s0);
    takes_ownership_0(s0.clone());
    takes_ownership_1(s1);

    println!("{s0}");
    println!("{s1}");

    let x = 100;

    makes_copy(x);

    println!("{x}");
}

fn takes_ownership_0(s: String) {
    println!("{s}");
}

fn takes_ownership_1(s: &str) {
    println!("{s}");
}

fn makes_copy(v: i32) {
    println!("{v}");
}

pub fn exmpl_3() {
    let s1 = String::from("hello");

    // let (s2, len) = calculate_length(s1);
    // println!("The length of '{s2}' is {len}.");

    // использование ссылки
    let len = calculate_length_with_reference(&s1);
    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();

    (s, len)
}

fn calculate_length_with_reference(s: &String) -> usize {
    s.len()
}

pub fn exmpl_4() {
    let mut s = String::from("Hello");

    let s1 = &mut s;
    // нельзя
    // let s2 = &mut s;
    // println!("{s1}, {s2}");

    // можно
    println!("{s1}");
    let s2 = &mut s;
    println!("{s2}");

    // можно
    change(&mut s);
    change_2(&mut s);

    println!("After {s}");
}

fn change(s: &mut String) {
    println!("Before {s}");
    s.push_str(", World!");
}

fn change_2(s: &mut String) {
    println!("Before 2 {s}");
    s.push_str("!!!");
}