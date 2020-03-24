mod reactions;
mod commands;

#[derive(Debug, Clone)]
pub struct Bot {
    token: String,
}

impl Bot {
    pub fn new(token: String) -> Self {
        Bot { token }
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    // Accessible in this module and this module's children modules
    fn internal_func(&self) {}

    pub(super) fn semi_internal_func(&self) {}
}

#[cfg(test)]
mod tests {
    use super::Bot;

    #[test]
    fn my_test1() {
        let bot1 = Bot::new(String::from("abc"));
        let bot2 = Bot { token: String::from("abc") };
        assert_eq!(bot1.token, bot2.token);
    }

    #[test]
    fn my_test2() {
        let bot = Bot::new(String::from("abc"));
        bot.internal_func();
        assert_eq!(2 + 2, 4, "oh no!");
    }
}
