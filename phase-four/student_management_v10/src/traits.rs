
use crate::input_parsing::user_input;
use crate::manager::StudentManager;
use crate::teacher::Teacher;
use crate::trait_functions::{update_score_func, student_list_func};
use crate::routing::load_database_persistent;
use crate::student::Student;
use crate::error::SystemError;

pub trait UpdateScore{
    fn update_score(&mut self);
}

impl UpdateScore for StudentManager{ 
    fn update_score(&mut self){
        reuseable_update_score(self);
    }
}

impl UpdateScore for Teacher{ 
    
    fn update_score(&mut self){
        let mut database = load_database_persistent();
        reuseable_update_score(&mut database);
    }
}

pub trait ClassList{
     fn list_student(&self);
}

impl ClassList for StudentManager{
     fn list_student(&self) {
        reusable_class_list(&self);
        
    }
}

impl ClassList for Teacher{
    fn list_student(&self) {
        let database=&load_database_persistent();
        reusable_class_list(&database);
    }
}

fn reuseable_update_score(database:&mut StudentManager){
    println!("Enter the user name");
    let name = user_input();
        match update_score_func(database, &name){
            Ok(()) => {
                println!("Successfully update score for {}", &name)
            },
            Err(e) => {
                println!("{}",e)
            }
        }
}

fn reusable_class_list(database:&StudentManager){
    let mut counter =0;
         match student_list_func(database){
            Ok(student_vector) => {
                for (count, student) in student_vector.iter().enumerate(){
                    counter +=1;
                    println!("================STUDENT {}=================", count + 1);
                    println!("name: {}\nage: {}\nscore: {}", student.name, student.age, student.score);
                }
                    println!("Total number of student: {}", counter);
            }

            Err(e) => {
                println!("{}", e)
            }
         } 
}

pub trait Summary{
    fn summary(&self)->Result<Student, SystemError>;
}

impl Summary for Teacher{
    fn summary(&self)->Result<Student, SystemError> {
        let database= load_database_persistent();
        match generic_summary(&database){
            Ok(student) =>{
                return Ok(student.clone())
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}

impl Summary for StudentManager{
    fn summary(&self)->Result<Student, SystemError> {
        match generic_summary(&self){
            Ok(student) =>{
                return Ok(student.clone())
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}

fn generic_summary(database:&StudentManager)->Result<&Student, SystemError>{
    println!("type the name of the student to view summary");
    let name:String = user_input();
    let student = database.students.values().find(|x| x.name.to_lowercase() == name).map(|x| x);
    match student{
        Some(student) => {
            Ok(student)
        }
        None => {
            Err(SystemError::StudentNotFound)
        }
    }

}