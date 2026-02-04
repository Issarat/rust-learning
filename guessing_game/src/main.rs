mod logic;
mod inputs;
use logic::{game_msg, check_guess_num};
use inputs::{input_umber_validate_check};
use rand::Rng;

fn main() {
    let _digit :u32 = 2;
    let secret_number :u32 = rand::thread_rng().gen_range(1..=99);
    loop {
        let input :u32 = input_umber_validate_check(_digit);
        let result_guess :String = check_guess_num(input, secret_number);

        println!("Guess the number!");
        println!("You guessed: {input}");
        println!("{}", result_guess); // พิมพ์ผลลัพธ์ (Too small / Too big / You win)

        // ตรวจสอบว่าข้อความที่ส่งกลับมาคือคำว่าชนะหรือไม่
        if result_guess == game_msg::YOU_WIN {
            println!("Congratulations! 🏆");
            break; // สั่งหยุด loop ทันที
        }
    }
}
