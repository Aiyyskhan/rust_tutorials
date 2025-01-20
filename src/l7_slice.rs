use std::iter;


pub fn exmpl_1() {
    let s = String::from("hello world");

    let hello = &s[..5];
    let world = &s[6..];

    println!("{hello}");
    println!("{world}");
}

pub fn exmpl_2() {
    let s = String::from("valar morgulis");
    let res = first_word(&s);
    println!("{res}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

pub fn exmpl_3() {
    let arr = [10,20,30,40,50,60];

    let res = &arr[..3];

    for (idx, it) in res.iter().enumerate() {
        println!("{idx} - {it}");
    }
}