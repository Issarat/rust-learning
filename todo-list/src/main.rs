mod services;
use chrono::Local; 
use services::storages::manage::view::{get_task_log,filter_n_select_log};
use crate::services::tasks::model::Task; 
use services::tasks::manage::{add::add_new_task_log};
use services::tasks::model::*;
#[derive(Debug)]
enum MenuState {
    View,
    Add,
    Edit,
    EndProcess
}

fn main() {
    loop {
        // สมมติว่าฟังก์ชัน get_user_input() คืนค่าเป็น MenuState
        clear_terminal();
        let state = get_user_input(); 

        match state {
            MenuState::View => {
                get_task_log(None);
                filter_n_select_log();
                //println!("--- Showing all tasks ---");
                // logic สำหรับดูข้อมูล
            }
            MenuState::Add => {
                println!("--- Adding a new task ---");
                let date = Local::now().format("%Y-%m-%d").to_string();
                let filename = format!("{}.json", date);
                let tasks: Vec<Task> = vec![
                    Task { id: 1, title: "Learn Rust".to_string(), status: TaskStatus::Todo },
                    Task { id: 2, title: "Write Code".to_string(), status: TaskStatus::InProgress },
                ];

               add_new_task_log(tasks, filename);
            }
            MenuState::Edit => {
                println!("--- Editing a task ---");
                // logic สำหรับแก้ไข
            }
            MenuState::EndProcess => {
                println!("Exiting program... Bye!");
                break; // ออกจาก loop
            }
        }
    }
}

// #region local fn
fn get_user_input() -> MenuState {
    loop { // วนลูปที่นี่จนกว่าจะได้ค่า 1, 2, 3 หรือ 4
        let mut input: String = String::new();
        println!("Select: (1) View, (2) Add, (3) Edit, (4) Exit");
        
        if std::io::stdin().read_line(&mut input).is_ok() {
            match input.trim() {
                "1" => return MenuState::View,
                "2" => return MenuState::Add,
                "3" => return MenuState::Edit,
                "4" => return MenuState::EndProcess,
                _ => {
                    // ถ้ากรอกผิด ไม่ต้องทำอะไร (ไม่ต้องพิมพ์ Invalid choice)
                    // แค่ล้างหน้าจอและพิมพ์ Select ใหม่
                    clear_terminal(); 
                    continue; 
                }
            }
        }
    }
}

fn clear_terminal() {
    // ส่ง ANSI Escape Code ไปที่ Terminal เพื่อล้างหน้าจอและเลื่อนเคอร์เซอร์ไปบนสุด
    print!("{esc}c", esc = 27 as char); 
}
// #endregion