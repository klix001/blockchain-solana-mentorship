
use crate::input_parsing::user_input;
use crate::manager::StudentManager;
use crate::teacher::Teacher;
use crate::trait_functions::{update_score_func, student_list_func};
use crate::routing::load_database_persistent;

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

pub trait ListStudent{
     fn list_student(&self);
}

impl ListStudent for StudentManager{
     fn list_student(&self) {
        reusable_class_list(&self);
        
    }
}

impl ListStudent for Teacher{
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
    let mut counter =1;
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