fn main() {
    let my_bool = &42u8 as *const u8 as *const bool;
    unsafe {
        println!("{}", *my_bool);
    }

    /*
    let mut vec = vec![1, 2, 3];

    let vec_ptr = &mut vec as *mut Vec<_>;

    unsafe {
        let first = (*vec_ptr).get_unchecked_mut(0); // < unsafe but fine
        (*vec_ptr).clear();
        *first += 1;
    }
    */
}
