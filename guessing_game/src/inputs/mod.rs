use std::io::{self, Write};

pub fn input_umber_validate_check(num_digits: u32) -> u32
{
    let mut guess = String::new();
    loop {
        guess.clear(); // สำคัญมาก! ต้องล้างค่าเก่าทิ้งก่อนรับค่าใหม่ใน Loop
        print!("Please input your number ({} digits): ", num_digits);
        io::stdout().flush().unwrap(); // เพื่อให้ข้อความ print! แสดงผลทันที

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 1. ตัดช่องว่าง/ขึ้นบรรทัดใหม่ทิ้ง
        let trimmed = guess.trim();

        // 2. ตรวจสอบเงื่อนไข: เป็นตัวเลขหรือไม่ และ มี 2 หลักหรือไม่
        match trimmed.parse::<u32>() {
            Ok(parsed_num) if trimmed.len() <= num_digits as usize => {
                println!("Valid input: {}", trimmed);
                return parsed_num; // ใช้ return เพื่อส่งค่าออกจากฟังก์ชันทันที
            }
            _ => {
                println!("❌ Invalid! Please enter exactly {} digits.", num_digits);
                // ไม่ต้องทำอะไร มันจะวนลูปกลับไปรับค่าใหม่เอง
            }
        }
    }
}