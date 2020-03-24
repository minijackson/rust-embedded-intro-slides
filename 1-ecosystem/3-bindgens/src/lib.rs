#[no_mangle]
pub extern fn say_hello() {
    println!("Hello from Rust!");
}

#[repr(C)]
#[derive(Debug)]
pub struct Point2D {
    x: u32,
    y: u32,
}

#[no_mangle]
pub extern fn print_point(point: &Point2D) {
    println!("Rust using the Debug trait: {:?}", point);
}

#[no_mangle]
pub extern fn make_vec(size: usize, value: i32) -> *mut Vec<i32> {
    let mut vec = Box::new(Vec::new());
    vec.resize(size, value);
    Box::into_raw(vec)
}

#[no_mangle]
pub extern fn print_vec(vec: &Vec<i32>) {
    println!("Rust printing the Vec: {:?}", vec);
}

#[no_mangle]
pub unsafe extern fn free_vec(vec: *mut Vec<i32>) {
    println!("Rust Freeing a Vec");
    Box::from_raw(vec);
}
