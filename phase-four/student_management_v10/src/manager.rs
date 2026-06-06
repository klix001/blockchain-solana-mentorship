use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::student::Student;
use crate::input_parsing::user_input;
use crate::teacher::Teacher;

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentManager{
    pub students:HashMap<String, Student>,
    pub teachers:HashMap<String, Teacher>,
}

 #[derive(Debug, Clone)]
pub struct ClassReport{
        pub pass:Vec<String>,
        pub failed:Vec<String>,
        pub scholarship:Vec<String>,
    }

    

impl StudentManager{
    pub fn new()->Self{
        Self{
            students:HashMap::new(),
            teachers:HashMap::new(),
            
        }
    }

    pub fn admin(&mut self){
        self.admin_menu();
        
    }

    pub fn teacher_menu(&mut self){
        println!("Enter your id");
        let teacher_id = user_input().to_uppercase();
        match self.teachers.get_mut(&teacher_id){
            Some(teacher) => {
                teacher.teacher_menu();
            },
            None => println!("{} not a registered ID", teacher_id),
        }
    }
    pub fn student_menu(&self){
        println!("Enter you name to access your profile");
        let user_name:String = user_input().to_lowercase();
        match self.students.get(&user_name){
            Some(student) => {
                    println!("************{} PERSONAL DASHBOARD****************", student.name.to_uppercase());
                    student.menu();                
            },
            None => {
                println!("{} not found in record", user_name);
            }
        }
    }

    
}