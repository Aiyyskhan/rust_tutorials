pub fn exmpl() {
    // кортеж (tuple)
    // let a: (i32, f32, u8) = (500, 3.14, 0);
    // let el: f32 = a.1;

    // println!("Tuple element: {el}");

    // Массив (array)
    // let arr: [u32; 4] = [10, 39, 0, 1];
    let mut arr: [u32; 4] = [0; 4];

    let mut idx: usize = 0;

    let mut el: u32 = arr[idx];
    println!("The value of the element at index {idx} is: {el}");
    
    idx = 1;

    arr[idx] = 42;

    el = arr[1];
    println!("The value of the element at index {idx} is: {el}");
}