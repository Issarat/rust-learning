pub const Choice: [(&str, &str); 3] = [
    ("R", "ROCK 🪨"),
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

pub fn get_choice(input: String) -> String
{
        CHOICE
        .iter()
        .find(|(key, _)| key == &upper_input) 
        // ถ้าเจอ จะได้ Some((key, value)) ถ้าไม่เจอจะได้ None
        .map(|(_, value)| value.to_string()) 
        // ถ้าเป็น None ให้ส่งข้อความแจ้งเตือนกลับไป (เหมือนการทำ Default Value)
        .unwrap_or_else(|| "Invalid Choice!".to_string())   
}

pub fn validate_choice(input : String) -> bool {
    //input is S P R
    let upper :String = input.to_uppercase();
    CHOICE.iter().any(|&(key, _)| key == upper)
}

pub fn get_rand_choice() -> String{
    let mut rng = thread_rng();
    // สุ่มเลือก 1 Tuple จาก Array
    // .choose() จะคืนค่าเป็น Option เพราะ Array อาจจะว่างได้
    if let Some(picked) = CHOICE.choose(&mut rng) {
        let (key, value) = picked;
    }
}