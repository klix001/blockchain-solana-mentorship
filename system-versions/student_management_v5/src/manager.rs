use serde::{Serialize, Deserialize};
use crate::student::Student;
use crate::input_parsing::user_input;
use crate::teacher::Teacher;

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentManager{
    pub students:Vec<Student>,
    pub teachers:Vec<Teacher>,
}

impl StudentManager{
    pub fn new()->Self{
        Self{
            students:vec![],
            teachers:vec![]
            
        }
    }

    pub fn admin(&mut self){
        self.admin_menu();
        
    }

    pub fn teacher_menu(&mut self){
        println!("Enter your id");
        let teacher_id = user_input().to_uppercase();
        match self.teachers.iter_mut().find(|x| x.id == teacher_id){
            Some(teacher) => {
                teacher.teacher_menu();
            },
            None => println!("{} not a registered ID", teacher_id),
        }
    }
    pub fn student_menu(&self){
        println!("Enter you name to access your profile");
        let user_name:String = user_input().to_lowercase();
        match self.students.iter().find(|x| x.name.to_lowercase() == user_name){
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