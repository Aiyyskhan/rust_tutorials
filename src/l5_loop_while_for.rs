pub fn exmpl() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter;
        }
    };

    println!("Result: {result}");
}

pub fn exmpl_1() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

pub fn exmpl_2() {
    let mut num = 3;

    while num != 0 {
        println!("Number = {num}");

        num -= 1;
    }

    println!("LIFTOFF!!!");
}

pub fn exmpl_3() {
    // перебор массива
    // let arr = [10, 20, 30, 40, 50];
    // for el in arr {
    //     println!("The value is {el}");
    // }

    // Range
    for num in (1..4).rev() {
        println!("{num}");
    }
    println!("LIFTOFF!!!");
}