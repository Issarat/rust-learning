use std::io::{self, Write};
use colored::Colorize;

use crate::constants::choice::*;
use crate::constants::result::game_msg;
use crate::logic::check_result;

pub fn input_choice()
{
    let mut input = String::new();
    let menu = get_menu();
    loop {
        input.clear(); // สำคัญมาก! ต้องล้างค่าเก่าทิ้งก่อนรับค่าใหม่ใน Loop
        print!("Please input your choice [{}] : ", menu);
        io::stdout().flush().unwrap(); // เพื่อให้ข้อความ print! แสดงผลทันที

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_upper = input.trim().to_uppercase();
        let check = validate_choice(trimmed_upper.as_str());

        if !check { continue }
        let p_choice_text = get_choice(trimmed_upper.as_str());
        let (s_key,s_value) = get_rand_choice();

        let res_msg = check_result(trimmed_upper.as_str(), s_key.as_str());    

        println!("--------------------------");
        println!("You choice is : {}", p_choice_text);
        println!("Bot choice is : {}", s_value);
        println!("RESULT: {}", res_msg);
        println!("--------------------------");

        if res_msg == game_msg::WIN.green() {break;}
        println!("Try again to win!");
    }
}
