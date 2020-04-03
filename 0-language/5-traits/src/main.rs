#![allow(dead_code, unused_variables)]

trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn print_summary<T: Summary>(thing: &T) {
    println!("{}", thing.summarize());
}

// Same thing
fn print_summary2<T>(thing: &T)
where
    T: Summary,
{
    println!("{}", thing.summarize());
}

fn main() {
    let article = NewsArticle {
        headline: String::from("L'OMS confirme qu'un bon gros kebab sauce samouraï est une protection efficace contre le coronavirus"),
        location: String::from("France"),
        author: String::from("La Rédaction du Gorafi"),
        content: String::from("..."),
    };

    print_summary(&article);

    let tweet = Tweet {
        username: String::from("realDonaldTrump"),
        content: String::from("So much FAKE NEWS!"),
        reply: true,
        retweet: false,
    };

    print_summary(&tweet);
}
