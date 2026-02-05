use std::fs;
use colored::*;
use chrono::NaiveDate;
use crate::services::tasks::model::*; 
//read file on steam
use std::fs::File;
use std::io::BufReader;
use serde_json;


/// ดู list ใน folder task log 
/// - `filter`: (Nullable) หากระบุ จะแสดงเฉพาะไฟล์ที่มีคำค้นหานั้น
pub fn get_task_log(filter: Option<String>) {
    let path = "./task_log";

    match fs::read_dir(path) {
        Ok(entries) => {
            println!("\n📂 {}", "Directory Listing: task_log".bright_cyan().bold());
            println!("{}", "---------------------------".bright_black());

            // 1. รวบรวม entries ที่อ่านได้สำเร็จลงใน Vec
            let valid_entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();

            // 2. เช็คว่ามีไฟล์ไหมจากขนาดของ Vec
            if valid_entries.is_empty() {
                println!("📄 {}", "Empty Task Log".bright_black().italic());
            } else {
                for entry in valid_entries {
                    let name = entry.file_name().to_string_lossy().into_owned();

                    // ตรรกะการ Filter
                    match &filter {
                        Some(f) if !name.contains(f) => continue, // ถ้ามี Filter และไม่ตรง ให้ข้าม
                        _ => println!("📄 {}", name), // กรณีอื่นๆ ให้แสดงผล
                    }
                }
            }
            println!("{}", "---------------------------\n".bright_black());
        }
        Err(e) => println!("{} {}", "❌ Error:".red(), e),
    }
}

/// ใช้ serch file
pub fn filter_n_select_log() {
    let mut input: String = String::new();
    let format_pattern: &str = "%Y-%m-%d";
    loop{
            input.clear();
            println!("👉 กรุณาพิมพ์ชื่อไฟล์ที่ต้องการค้นหาหรือเปิด (เช่น 2026-02-05, 2026-02 , 05) b:เพื่อย้อนกลัย : ");
            std::io::stdin().read_line(&mut input).unwrap();
            let trim_input: &str = input.trim();
            if trim_input.to_lowercase() == "b" {break}; 
            match NaiveDate::parse_from_str(trim_input, format_pattern) {
                Ok(date_obj) => {
                    // ใช้ date_obj.to_string() เพื่อให้มั่นใจว่าได้ฟอร์แมต "yyyy-mm-dd" ที่สะอาด
                    let filename: String = format!("{}.json", date_obj); 

                    let tasks: Vec<Task> = find_task_log(&filename);

                    if tasks.is_empty() {
                        println!("{}", "⚠️ ไม่พบไฟล์ หรือไม่มีข้อมูลในไฟล์นี้".yellow());
                    } else {
                        println!("\n📋 รายการงานจากวันที่: {}", date_obj.format("%d/%m/%Y").to_string().cyan());
                        println!("{}", "---------------------------------".bright_black());
                        
                        for task in tasks {
                            let status_icon: &str = if task.status == TaskStatus::Done { "✅" } else { "⏳" };
                            println!("{} [{}] - {}", status_icon, task.id, task.title);
                        }
                        println!("{}", "---------------------------------".bright_black());
                    }
                }
                Err(_) => {
                    get_task_log(Some(trim_input.to_string()));
                }
            }
    }
}

///serch ใน task_log folder 
pub fn find_task_log(filename: &str) -> Vec<Task> {
    let path: String = format!("task_log/{}", filename);

    let file: File = match File::open(&path) {
        Ok(f) => f,
        Err(_) => {
            println!("❌ ไม่พบไฟล์: {}", path);
            return vec![];
        }
    };

    let reader = BufReader::new(file);
    let tasks: Vec<Task> = serde_json::from_reader(reader)
        .expect("❌ รูปแบบไฟล์ JSON ไม่ถูกต้อง");
    tasks
}