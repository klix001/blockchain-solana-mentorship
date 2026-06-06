
use crate::error::SystemError;
use crate::manager::StudentManager;
use crate::input_parsing::{user_input, read_input};
use crate::model::AdminAction;
use crate::storage::save;
use crate::student::Student;
use crate::teacher::Teacher;
use crate::traits::{ListStudent, UpdateScore};

impl StudentManager{
    pub fn admin_menu(&mut self){
        
        loop{
            println!("*************STUDENT MANAGER MENU****************");
            println!("Enter the following option");
            println!("Option 1: add student");
            println!("Option 1: add teacher");
            println!("Option 2: class list");
            println!("Option 3: class average");
            println!("Option 4: delete student");
            println!("Option 5: update score");
            println!("Option 6: pass report");
            println!("Option 7: eligibility report");
            println!("Option 8: exit");
            let admin_action:AdminAction = loop{
                match user_input().parse::<AdminAction>(){
                    Ok(action) => break action,
                    Err(_) => {
                        println!("Enter a valid option");
                        continue;
                    }
                }
            };
            match admin_action {
                AdminAction::ClassAverage => {
                    match self.class_average(){
                        Ok(score) => {
                            println!("Average: {}",score)
                        }
                        Err(e) => {
                            println!("Error: {}", e)
                        }
                    }
                }
                AdminAction::PassStatus => {
                    self.class_report();
                }
                AdminAction::UpdateScore => {
                    self.update_score();
                }
                AdminAction::Delete =>{
                    self.delete_student();
                }
                AdminAction::AddStudent => {
                    match self.add_student(){
                        Ok(student) => {
                            println!("{} Successfully added to the student record", student.name)
                        }
                        Err(e) => {
                            println!("Unable to add student: {}", e)
                        }
                    }
                }
                AdminAction::ViewStudent => {
                    self.view_students();
                }
                AdminAction::AddTeacher => {
                    match self.add_teacher(){
                        Ok(id) =>{
                            println!("teacher with {} ID Successfully registered ", id)
                        }
                        Err(e) => println!("Error: {}", e)
                    }
                }
                AdminAction::Exit => break,
                _ => {
                    println!("Action not implemented yet");
                }              
            }
        };
    }

    fn validate_student(&self)->Result<Student, SystemError>{
        println!("Enter student name");
        let name = user_input();
        println!("Enter student age");
        let age = read_input()?;
        println!("Enter student score");
        let score = read_input()?;

        match (age, score, name.is_empty()){
            (a,s, false) if a>=16 && s<=100.0 => {
            let new_student:Student= Student::new(name, age, score);
                Ok(new_student)
            }
            _ => return Err(SystemError::AddStudentFailure)
        }
    }

    fn add_student(&mut self)->Result<Student, SystemError>{
        let new_student = self.validate_student()?;
        self.students.push(new_student.clone());
        let _= save(&self);
        return Ok(new_student);
    }

    fn add_teacher(&mut self)->Result<String, SystemError>{
        println!("Enter teacher ID");
        let id = user_input().to_uppercase();
        println!("Enter department");
        let department = user_input();
        println!("Enter course code");
        let course = user_input().to_uppercase();

        match (&id, &department, &course){
            (id, department, course) if id.is_empty() 
            && department.is_empty() 
            && course.is_empty() => {
                return Err(SystemError::AddTeacherFailure)
            }
            _ => {
            let new_teacher:Teacher= Teacher::new(id.clone(), department, course);
            self.teachers.push(new_teacher);
            let _= save(&self);
            Ok(id)
            }
        }   
    }

    fn view_students(&self){
        self.list_student();
    }

    fn class_average(&self)->Result<f64, SystemError>{
        let scores:Vec<f64> = self.students.iter().map(|x| x.score).collect();
        if scores.is_empty(){
            return Err(SystemError::EmptyDatabase);
        }else{
            let total:f64 = scores.iter().sum();
            let average: f64 = total/scores.len() as f64;
            Ok(average)
        }
    }

    fn delete_student(&mut self){
        println!("Enter the student name");
        let name = user_input();
        let search = self.students.iter().position(|x| x.name.to_lowercase() == name.to_lowercase());
        match search {
            Some(index) => {
                self.students.remove(index);
                let _ = save(&self);
                println!("{} Successfully deleted\n", name);
            }
                None => println!("Student not found in database"),
        }
    }

    // I WILL REWRITE THIS PART LATTER I JUST WANT IT TO COMPILE NOW( seperate logic from display for testability)
    fn class_report(&self){
        let pass_student:Vec<String> = self.students.iter()
        .filter(|x| x.score >= 50.0)
        .map(|x| x.name.clone())
        .collect();
        self.report_formater(pass_student, "Passed");
        

        let failed:Vec<String> = self.students.iter()
        .filter(|x| x.score < 50.0)
        .map(|x| x.name.clone())
        .collect();
        self.report_formater(failed,"Failed");

        let scholarship:Vec<String> = self.students.iter()
        .filter(|x| x.score >= 85.0)
        .map(|x| x.name.clone())
        .collect();
        self.report_formater(scholarship, "scholarship Student");
    }

    fn report_formater(&self,student_status:Vec<String>, label:&str){
        if !student_status.is_empty(){
            println!("\n{}: ", label);
            let mut count = 0;
            for student in student_status{
                count +=1;
                println!("{} {}", count, student);
            }
            println!("{}: {}\n", label, count);
        }
        else{
            println!("{}: No student {}\n", label, label);
        }
    }
}