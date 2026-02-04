use rand::seq::IndexedRandom; // สำหรับ rand 0.9.1 ใช้ตัวนี้แทน SliceRandom
use rand::rng;

pub const CHOICE: [(&str, &str); 3] = [
    ("R", "ROCK "),
    ("P", "PAPER 🧻"),
    ("S", "SCISSOR ✂️"),
];

pub fn get_menu() -> String
{
        CHOICE
        .iter()
        .map(|(key, value)| format!("{key} : {value}")) // เหมือน $"{key} : {value}"
        .collect::<Vec<String>>()                       // แปลงสายพานเป็น List (เหมือน .ToList())
        .join(" | ")
}

pub fn get_choice(input: &str) -> String
{
        CHOICE
        .iter()
        .find(|(key, _)| key == &input) 
        // ถ้าเจอ จะได้ Some((key, value)) ถ้าไม่เจอจะได้ None
        .map(|(_, value)| value.to_string()) 
        // ถ้าเป็น None ให้ส่งข้อความแจ้งเตือนกลับไป (เหมือนการทำ Default Value)
        .unwrap_or_else(|| "Invalid Choice!".to_string())   
}

pub fn validate_choice(input : &str) -> bool {
    //input is S P R
    let upper :String = input.to_uppercase();
    CHOICE.iter().any(|&(key, _)| key == upper)
}

pub fn get_rand_choice() -> (String, String) {
    let mut rng = rng();

    // .choose() จะกลับมาใช้งานได้ตามปกติ
    let (key, value) = CHOICE
        .choose(&mut rng)
        .expect("CHOICE array should not be empty");

    (key.to_string(), value.to_string())
}