use crate::manager::StudentManager;
use crate::teacher::Teacher;
use crate::trait_functions::{update_score_func, student_list_func};
use crate::routing::load_database_persistent;


pub trait UpdateScore{
    fn update_score(&mut self);
}

impl UpdateScore for StudentManager{ 
    fn update_score(&mut self){
        update_score_func(self);
    }
}

impl UpdateScore for Teacher{ 
    fn update_score(&mut self){
        let mut database = load_database_persistent();
        update_score_func(&mut database);
    }
}

pub trait ListStudent{
     fn list_student(&self);
}

impl ListStudent for StudentManager{
     fn list_student(&self) {
        student_list_func(self);
    }
}

impl ListStudent for Teacher{
    fn list_student(&self) {
        let database=&load_database_persistent();
        student_list_func(database);
    }
}