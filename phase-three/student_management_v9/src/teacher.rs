use serde::{Serialize, Deserialize};
use crate::input_parsing::user_input;
use crate::model::{TeacherAction};
use crate::routing::load_database_persistent;
use crate::student::Student;
use crate::traits::{ClassList, UpdateScore, Summary};

#[derive(Debug, Serialize, Deserialize)]
pub struct Teacher{
pub id:String,
pub department:String,
pub course:String,
}

impl Teacher{
     pub fn new(id:String, department:String, course:String)->Self{
        Self { id,
            department, 
            course 
        }
    }

    pub fn teacher_menu(&mut self){
        println!("==============TEACHER MANAGEMENT PORTAL==================");
        loop {
            println!("======================Teacher Menu ===================");
            println!("Enter the following Options");
            println!("Option 1: update score");
            println!("Option 2: class list");
            println!("Option 3: evaluation");
            println!("Option 4: class stat");
            println!("Option 5: generate report");
            println!("Option 6: student summary");
            println!("Option 7: exit");


            let action:TeacherAction = loop{
                match user_input().parse::<TeacherAction>(){
                    Ok(action) => break action,
                    Err(_) => {
                        println!("Enter a valid operation");
                        continue;
                    }
                }
            };
            match action{
                TeacherAction::UpdateScore => {
                    self.update_score();
                }
                TeacherAction::Summary =>{
                    match self.summary(){
                        Ok(student)=> {
                            println!("name: {} \nage: {} \nscore: {}", student.name, student.age, student.score)
                        }
                        Err(e)=> {
                            println!("Error: {}",e)
                        }
                    }
                }
                TeacherAction::ClassList => {
                    self.list_student();
                }
                TeacherAction::ClassStatistics => {
                    self.class_stat();
                }
                TeacherAction::Exit => break,
                _ => println!("Invalid Operation"),
            }
         
        };
    }

    fn class_stat(&self){
        let database = load_database_persistent();
        let student_record:Vec<&Student> = database.students.iter().collect();
        
        let score:Vec<f64> = student_record.iter().map(|x| x.score).collect();
        let total: f64 = score.iter().sum();
        let max: Option<&f64> =score.iter().max_by(|a , b| { 
            a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
        });
        let min: Option<&f64> =score.iter().min_by(|a , b| { 
            a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
        });
        if score.is_empty(){
            println!("Student record is empty");
        }
        let average:f64 = total/student_record.len() as f64; 
        println!("====================Class Statistics==================");
        match max {
            Some(max) =>{
                println!("Max: {}", max)
            }
            None => println!("Max: database is empty")
        }

        match min {
            Some(min) =>{
                println!("Min: {}", min)
            }
            None => println!("Min: database is empty")
        }
        println!("average: {} \nClass total: {}", average, student_record.len());
        
    }
}