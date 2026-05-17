use std::io;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::io::{Read, Write};
use std::fs;

const DB_FILE:&str = "students_db.json";

fn main() {
    role_based_routing();
}

fn read_u32()->u32{
    let int_value:u32 = loop{
        match user_input().parse::<u32>(){
            Ok(int_value) => break int_value,
            Err(_) => {
                println!("Enter a valid integer");
                continue;
            }
        }
    };
    int_value
}

fn read_f64()->f64{
    let float_value:f64 = loop{
        match user_input().parse::<f64>(){
            Ok(float_value) => break float_value,
            Err(_) => {
                println!("Enter a valid decimal float");
                continue;
            }
        }
    };
    float_value
}

fn user_input()->String{
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).expect("failed to read input");
    input.trim().to_string()
}

fn role_based_routing(){
    let mut manager = StudentManager::load_db().unwrap_or_else(|e|{
        eprintln!("Unable to load database {e}");
        StudentManager::new()
    });
    println!("Enter user role: Student / Student manager");
    let role:Route = loop{
        match user_input().parse::<Route>(){
            Ok(role) => break role,
            Err(_) => {
                println!("Enter a valid role");
                continue;
            }
        }
    };

    match role {
        Route::StudentManager => manager.admin_menu(),
        Route::Student => {
            manager.student_menu();
        },
    }
    
}

// ROUTING ENUM
enum Route{
    StudentManager,
    Student,
}

impl FromStr for Route{
    type Err=String;
    fn from_str(role:&str)->Result<Self, Self::Err>{
        match role.to_lowercase().as_str(){
            "student manager" => Ok(Route::StudentManager),
            "student" => Ok(Route::Student),
            _ => Err(format!("Enter a valid role")),
        }
    }
}

// STUDENT STRUCT
#[derive(Debug, Serialize, Deserialize)]
struct Student{
    name:String,
    age:u32,
    score:f64,
}

impl Student{
    fn new(name:String, age:u32, score:f64)->Self{
        Self{
            name, age, score,
        }
    } 
}

// STUDENT MANAGER STRUCT
#[derive(Debug, Serialize, Deserialize)]
struct StudentManager{
    students:Vec<Student>,
}

enum AdminAction{
    AddStudent,
    ViewStudent,
    ClassAverage,
    Delete,
    UpdateScore,
    PassStatus,
    EligibilityReport,
    Exit,

}

impl FromStr for AdminAction{
    type Err = String;

    fn from_str(action:&str)->Result<Self, Self::Err>{
        match action.to_lowercase().as_str(){
            "add student" => Ok(AdminAction::AddStudent),
            "view student" => Ok(AdminAction::ViewStudent),
            "average"      => Ok(AdminAction::ClassAverage),
            "delete student" => Ok(AdminAction::Delete),
            "update score" => Ok(AdminAction::UpdateScore),
            "pass report" => Ok(AdminAction::PassStatus),
            "eligibility report" => Ok(AdminAction::EligibilityReport),
            "exit" => Ok(AdminAction::Exit),

            _ => Err(format!("Invalid action")),
        }
    }
}

impl StudentManager{
    fn new()->Self{
        Self{
            students:vec![],
        }
    }

    fn add_student(&mut self){
        println!("Enter student name");
        let name:String = user_input();

        println!("Enter student age");
        let age:u32 = read_u32();

        println!("Enter student score");
        let score:f64 = read_f64();

        let new_student:Student = Student::new(name, age, score,);
        self.students.push(new_student);
        self.save();
    }

    fn search_user(&self, user:String)->Option<&Student>{
        let result = self.students.iter().find(|x| x.name == user);
        match result {
            Some(result) => {
                Some(result)
            },
            None => {
                println!("user {} not found in record",user);
                None
            },
        }
    }

    fn student_menu(&self){
        println!("Enter your name to login");
        let user = user_input();
        let user_profile = self.search_user(user).unwrap();
        println!("{:#?}", user_profile);
    
    }

        fn admin_menu(&mut self){
            loop{
            println!("***********ADMIN MENU***************");
            println!("Enter the following options");
            println!("Option 1: add student");
            println!("Option 2: view student");
            println!("Option 3: average");
            println!("Option 4: delete student");
            println!("Option 5: update score");
            println!("Option 6: pass report");
            println!("Option 7: eligibility report");
            println!("Option 8: exit");
            
            let action:AdminAction = loop{
                match user_input().parse::<AdminAction>(){
                Ok(action) => break action,
                Err(_) => {
                    println!("Enter a valid option");
                    continue
                }
            }
        };
        match action {
    AdminAction::AddStudent => {
        self.add_student();
    }
    AdminAction::ViewStudent => {
        match self.view_student(){
            Some(student) => println!("{:#?}",student),
            None => println!("the record is empty"),
        }
    }
    AdminAction::ClassAverage => {
        match self.class_average(){
            Some(average) => println!("{:.2}", average),
            None => println!("record empty cannot compute average")
        }
    }
    AdminAction::Delete =>{
        println!("Enter the name of the student you want to delete");
        let user:String = user_input();
        self.delete_user(user);
    }
    AdminAction::UpdateScore => {
        println!("Enter the name of student score to be updated");
        let user = user_input();
        println!("Enter new score");
        let score = read_f64();
        match self.update_score(&user, score){
            Some(score) => {
                println!("Score has been successfully updated for {}", user)
        },
            None => {
                println!("Student not found");
            },
        }
    } 
    AdminAction::PassStatus => {
        match self.pass_report(){
            Some(report) => report.iter().for_each(|x| println!("{}", x)),
            None => println!("database is empty"),

        }
        
    }   
    AdminAction::EligibilityReport => {
        match self.eligibility(){
            Some(students) => students.iter().for_each(|x| println!("{}",x)),
            None => println!(""),
        }
    }
    AdminAction::Exit => {
        break;
        println!("Enter a valid option");
                },
        
        }
    };

    }


    fn load_db()->Result<Self, Box<dyn std::error::Error>>{
        if !Path::new(DB_FILE).exists(){
            let database = StudentManager::new();
            database.save()?;
            return Ok(database);

        }
        let students = fs::read_to_string(DB_FILE)?;
        let parsed = serde_json::from_str(&students)?;
        Ok(parsed)
        
    }

    fn save(&self)->Result<(), Box<dyn std::error::Error>>{
        let json = serde_json::to_string_pretty(&self)?;
        fs::write(DB_FILE, json)?;
        Ok(())
    }

    fn view_student(&self)->Option<Vec<&Student>>{
        if self.students.is_empty(){
            return None;
        }
        Some(self.students.iter().collect())
}

    fn class_average(&self)->Option<f64>{
        if self.students.is_empty(){
            return None;
        }
        let total:f64 = self.students.iter().map(|x| x.score).sum();
        Some(total/self.students.len() as f64)
    }

    fn delete_user(&mut self, user:String){
        match self.students.iter().position(|x| x.name==user){
            Some(index) => {
                self.students.remove(index);
                self.save();
            }
            None => ()
        };
    }

    fn update_score(&mut self, user:&str, score:f64)->Option<f64>{
        if self.students.is_empty(){
            return None;
        }
        let student = self.students.iter_mut().find(|x| x.name == user)?;
        student.score = score;
        let updated_score = student.score;
        self.save();
        Some(updated_score)
    }   
    fn pass_report(&self)->Option<Vec<String>>{
        if self.students.is_empty(){
            return None;
        }
        let pass_student:Vec<String> = self.students.iter().filter(|x| x.score >=50.0).map(|x| x.name.clone()).collect();
        
        let failed_student:Vec<String> = self.students.iter().filter(|x| x.score < 50.0).map(|x| x.name.clone()).collect();
        
        let honoured_student:Vec<String> = self.students.iter().filter(|x| x.score >= 85.0).map(|x| x.name.clone()).collect();

        let group_format = |label:&str, group:&Vec<String>|{
            match group.is_empty(){
                true => format!("{}: 0 student {}", label, label),
                false => format!("{}: {}", label, group.join(", ")),
            }
        };
        let students_report = vec![
            group_format("passed", &pass_student),
            group_format("failed", &failed_student),
            group_format("honoured", &honoured_student),

        ];
        Some(students_report)

    }
    fn eligibility(&self)->Option<Vec<String>>{
        if self.students.is_empty(){
            return None;
        }
        let ineligibile_student:Vec<String> = self.students.iter().filter(|x| x.score < 50.0).map(|x| x.name.clone()).collect();
        let scholarship_student:Vec<String> = self.students.iter().filter(|x| x.score >= 85.0).map(|x| x.name.clone()).collect();
        let eligibile_student:Vec<String> = self.students.iter().filter(|x| x.score >= 50.0).map(|x| x.name.clone()).collect();

        let group_formating = |label:&str, group:&Vec<String>|{
            match group.is_empty(){
                true => format!("{}: No student is eligible for {}", label, label),
                false => format!("{}: {}", label, group.join(", ")),
            }
        };
        let students_report=vec![
            group_formating("ineligible students", &ineligibile_student),
            group_formating("scholarship", &scholarship_student),
            group_formating("admission", &eligibile_student),
        ];
        Some(students_report)
    }
}