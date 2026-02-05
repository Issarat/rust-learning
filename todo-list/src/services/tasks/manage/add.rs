use serde_json;
use std::fs::{File};
use std::io::Write;
use crate::services::tasks::model::Task; 

/// write {list_task:Vec<Task> } file ที่ {filename:String}
pub fn add_new_task_log(list_task: Vec<Task>, filename : String){
    // 1. สร้างชื่อไฟล์จากวันที่ปัจจุบัน (yyyy-mm-dd)
    let directory = "task_log";
    let full_path = format!("{}/{}", directory, filename);

    // 2. แปลงเป็น JSON String
    let json_string = serde_json::to_string_pretty(&list_task)
        .expect("Failed to serialize tasks");

    // 3. บันทึกไฟล์
    let mut file = File::create(full_path).expect("Failed to create file");
    file.write_all(json_string.as_bytes()).expect("Failed to write data");

    println!("✅ Saved to task_log/{}.json", filename);
}