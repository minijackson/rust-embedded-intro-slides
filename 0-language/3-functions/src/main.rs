#![allow(dead_code, unused_variables)]

// These are equivalent
fn return_life() -> u8 {
    return 42;
}
fn return_life2() -> u8 {
    42
}

// These are equivalent
fn do_nothing() {}
fn do_nothing2() -> () {}

fn consume(value: Vec<i32>) {}
fn borrow(value: &Vec<i32>) {}
fn borrow_mutably(value: &mut Vec<i32>) {}

// Option<T> is either:
// - Some(thing: T)
// - None
fn try_get_first<T>(vec: &Vec<T>) -> Option<&T> {
    vec.get(0)
}

/// Panics if `vec` is empty
fn get_first<T>(vec: &Vec<T>) -> &T {
    try_get_first(vec).unwrap()
}

fn get_first2<'lifetime, T>(vec: &'lifetime Vec<T>) -> &'lifetime T {
    try_get_first(vec).unwrap()
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    println!("The answer is: {}!", return_life());

    let mut my_vec = vec![1, 2, 3];
    //  ^^^ note the mut here

    //consume(my_vec);
    borrow(&my_vec);
    borrow_mutably(&mut my_vec);
    //              ^^^ and here
    consume(my_vec);

    let my_vec = vec![1, 2, 3];
    dbg!(try_get_first(&my_vec)); // < Returns `Some(&1)`
    dbg!(get_first(&my_vec)); // < Returns `&1`

    let empty_vec: Vec<i32> = vec![];
    dbg!(try_get_first(&empty_vec)); // < Returns `None`

    //get_first(&empty_vec); // Panics!

    // What if the vector is destroyed while we hold a reference?
    /*
    let dangling_reference = {
        let my_vec = vec![1, 2, 3];
        get_first(&my_vec)
    };
    println!("{}", dangling_reference);
    */

    let a = String::from("abc");
    {
        let b = String::from("abcdef");

        let my_longest_str = {
            let a_ref: &str = &a;
            let b_ref: &str = &b;

            longest(a_ref, b_ref)
        };

        println!("{}", my_longest_str);
    }

    // Borrow rules intro
    /*
    let mut a = 42;
    let b = &a;
    let c = &a;
    println!("{}, {}, {}", a, b, c);
    /*
    let d = &mut a;
    println!("{}, {}", c, d);
    */
    */

    // What if we modify the vector while we hold a reference?
    /*
    {
        let mut my_vec = vec![1, 2, 3];
        let my_ref = get_first(&my_vec);
        my_vec.clear();
        println!("{}", my_ref); // Boom!
    }
    */

    /*
    {
        let mut my_vec = vec![1, 2, 3];
        let my_mut_ref = my_vec.get_mut(0).unwrap();
        my_vec.clear();
        println!("{}", my_mut_ref); // Boom!
    }
    */
}
