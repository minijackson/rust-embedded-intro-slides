#![allow(dead_code, unused_variables)]

mod asso_fun;

struct Person {
    name: String,
    address: String,
    age: u16,
    gender: Gender,
}

enum Gender {
    Male,
    Female,
    Other(String),
}

fn main() {
    start();
    //asso_fun::main();
}

fn start() {
    let this_guy = Person {
        name: String::from("Slartibartfast"),
        address: String::from("Magrathea"),
        age: 42,
        gender: Gender::Male,
    };

    println!("Hello: {}!", this_guy.name);

    let Person {
        name,
        age: this_guys_age,
        ..
    } = this_guy;

    println!("Hello: {}, {} years old!", name, this_guys_age);

    // ====================

    let this_gal = Person {
        name: String::from("Ella Fitzgerald"),
        address: String::from("Somewhere in the US"),
        age: 79,
        gender: Gender::Female,
    };

    match this_gal {
        Person { age: 42, .. } => println!("Perfect age!"),
        Person { age, .. } if age > 60 => println!("Quite old!"),
        Person { age, .. } => println!("Age: {}", age),
        //person => println!("Age: {}", person.age),
    }

    why_do_we_need_string();
}

fn why_do_we_need_string() {
    // Why do we need the new `String` type?

    let string1 = "hello";
    let string1: &str = "hello"; // < `char*` equivalent
    let string1: &'static str = "hello";

    // `std::string` equivalent
    let string2: String = String::from("hello");
}
