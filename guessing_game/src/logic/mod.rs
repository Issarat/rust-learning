use std::cmp::Ordering; 
pub mod game_msg {
    pub const TOO_SMALL: &str = "Too small! 📉";
    pub const TOO_BIG: &str = "Too big! 📈";
    pub const YOU_WIN: &str = "You win! 🎉";
}

pub fn check_guess_num(input: u32, secret_number: u32) -> String {
    match input.cmp(&secret_number) {
        Ordering::Less => game_msg::TOO_SMALL.to_string(),
        Ordering::Greater => game_msg::TOO_BIG.to_string(),
        Ordering::Equal => game_msg::YOU_WIN.to_string(),
    }
}