use serde::{Serialize, Deserialize};
use crate::student::Student;
use crate::input_parsing::{user_input, read_u32, read_f64};
use crate::storage::load_database;

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentManager{
    pub students:Vec<Student>
}

impl StudentManager{
    pub fn new()->Self{
        Self{
            students:vec![],
        }
    }

    pub fn student_manager(&mut self){
        loop{
            println!("*************STUDENT MANAGER MENU****************");
            println!("Enter the following option");
            
        };
        
    }

    pub fn student_menu(&self){
        let student_record = load_database().unwrap_or_else(|e| {
            eprintln!("Unable to load database {}",e);
            Self::new()
        });
        println!("Enter you name to access your profile");
        let user_name:String = user_input().to_lowercase();
        match student_record.students.iter().find(|x| x.name.to_lowercase() == user_name){
            Some(student) => {
                    println!("************{} PERSONAL DASHBOARD****************", student.name.to_uppercase());
                    student.student_menu(&student_record);                
            },
            None => {
                println!("{} not found in record", user_name);
            }
        }
    }
}