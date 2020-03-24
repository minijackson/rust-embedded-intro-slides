fn main() {
    let a = "world";
    let mut b = 42;
    let c = [1., 2., 3.]; // < All on the stack! Primitive
    let d = vec![1, 2, 3, 42]; // < All on the heap! Struct
    // ^^^ What is the type of these?

    println!("Hello, {}!", a);
    println!("b: {b}; c: {:?}; d: {:#?}", c, d, b = b);
    // See the documentation of std::fmt

    // How does println knows how to print
    // these different types?

    b = 1337;
    println!("New value of b: {}", b);

    let c = "something";
    println!("Value of second c: {}", c);
}
