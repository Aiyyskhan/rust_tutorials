pub fn exmpl() {
    let res = f0(1);
    println!("Result: {res}");

    let res = f1(true);
    println!("Result: {res}");
}

fn f0(a: usize) -> &'static str {
    if a < 5 {
        "condition was true"
    } else {
        "condition was false"
    }
}

fn f1(a: bool) -> i32 {
    // let num = if a { 10 } else { 30 };
    // num
    if a { 10 } else { 30 }
}