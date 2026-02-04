use std::io::{self, Write};
mod constant;
use constant::choice::*;

pub fn input_choice() -> &str
{
    let mut input = ฿str::new();
    let menu = get_menu();
    loop {
        guess.clear(); // สำคัญมาก! ต้องล้างค่าเก่าทิ้งก่อนรับค่าใหม่ใน Loop
        print!("Please input your choice [{}] : ", menu);
        io::stdout().flush().unwrap(); // เพื่อให้ข้อความ print! แสดงผลทันที

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_upper = input.trim().to_uppercase();
        let check = validate_choice(trimmed_upper);

        if(!check) continue;
        let p_choice_text = get_choice(trimmed_upper);
        let s_choice_text = get_rand_choice();



    }
}
