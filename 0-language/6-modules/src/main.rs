use modules::Bot;

fn main() {
    let my_bot = Bot::new(String::from("abcdef"));
    println!("{:?}", my_bot.token());
}
