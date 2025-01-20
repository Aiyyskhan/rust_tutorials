pub fn exmpl() {
    println!("Hello, world!");

    func_1();
    func_2(100);

    func_3();

    let y = func_4();

    println!("y: {y}");

    let x = func_5(10, 20);
    println!("The value of x is: {x}");
}

fn func_1() {
    println!("Func 1");
}

fn func_2(x: usize) {
    println!("Val = {x}");
}

fn func_3() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("y = {y}");
}

fn func_4() -> i32 {
    let y = {
        let x = 3;
        x + 1
    };
    y
}

fn func_5(a: usize, b: usize) -> usize {
    a * b
}