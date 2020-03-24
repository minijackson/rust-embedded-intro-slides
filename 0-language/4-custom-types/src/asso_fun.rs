struct Person {
    name: String,
    age: u16,
}

impl Person {
    fn greet(&self) {
        println!("Hello, {}!", self.name);
    }

    fn grow_older(&mut self) {
        println!("{} is growing older.", self.name);
        self.age += 1;
    }

    // `Consumes` self
    fn die(self) {
        println!("{} died at age {}", self.name, self.age);
    }

    fn with_name(name: String) -> Self { // here Self = Person
        Person {
            name, // < equivalent to `name: name`
            age: 0,
        }
    }
}

pub fn main() {
    let mut my_dude = Person {
        name: String::from("M.C. Escher"),
        age: 72,
    };

    my_dude.greet();
    println!("{} is {} years old.", my_dude.name, my_dude.age);
    my_dude.grow_older();
    println!("{} is {} years old.", my_dude.name, my_dude.age);
    my_dude.die();

    // Can't use M.C. Escher after that :'(

    // No matter, we can create other dudes
    let my_other_dude = Person::with_name(String::from("John Doe"));
    Person::greet(&my_other_dude);
}
