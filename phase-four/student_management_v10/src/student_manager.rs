
use crate::error::SystemError;
use crate::manager::{StudentManager, ClassReport};
use crate::input_parsing::{user_input, read_input};
use crate::model::AdminAction;
use crate::storage::save;
use crate::student::Student;
use crate::teacher::Teacher;
use crate::traits::{ClassList, Summary, UpdateScore};

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
            println!("Option 8: student summary");
            println!("Option 9: exit");
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
                AdminAction::Summary => {
                    match self.summary(){
                        Ok(student)=> {
                            println!("==================={} Summary================", student.name);
                            println!("name: {} \nage: {} \nscore: {}", student.name, student.age, student.score)
                        }
                        Err(e)=> {
                            println!("Error: {}",e)
                        }
                    }
                }
                AdminAction::PassStatus => {
                   match self.class_report(){
                    Ok(report) => {
                        let pass = report.pass;
                        self.reuseable_status("Pass List",pass, "No student made the pass mark");
                        let failed = report.failed;
                        self.reuseable_status("Fail List",failed, "All student passed"); 
                        let scholarship = report.scholarship;
                        self.reuseable_status("Scholarship List",scholarship, "There is no student eligible for a scholarship");
                    }
                    Err(e) => {
                        println!("Error: {}",e)
                    }
                   }
                }
                AdminAction::UpdateScore => {
                    self.update_score();
                }
                AdminAction::Delete =>{
                    match self.delete_student(){
                        Ok(name) => {
                            println!("Successfully deleted {} from student database",  name)
                        }
                        Err(e) => {
                            println!("Error: {}", e)
                        }
                    }
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
        if self.students.contains_key(&new_student.name){
            return Err(SystemError::StudentDuplicateError)
        }
        self.students.insert(new_student.name.clone(), new_student.clone());
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
            self.teachers.insert(new_teacher.id.clone(), new_teacher);
            let _= save(&self);
            Ok(id)
            }
        }   
    }

    fn view_students(&self){
        self.list_student();
    }

    fn class_average(&self)->Result<f64, SystemError>{
        if self.students.is_empty(){
            return Err(SystemError::EmptyDatabase);
        }else{
            let total:f64 = self.students.values().map(|x| x.score).sum();
            let average: f64 = total/self.students.len() as f64;
            Ok(average)
        }
    }

    fn delete_student(&mut self)->Result<String, SystemError>{
        println!("Enter the student name");
        let name = user_input();
        let search = self.students.get(&name);
        match search {
            Some(_) => {
                self.students.remove(&name);
                let _ = save(&self);
                Ok(name)
            }
                None => {
                    return  Err(SystemError::StudentNotFound);
                },
        }
    }

    // I WILL REWRITE THIS PART LATTER I JUST WANT IT TO COMPILE NOW( seperate logic from display for testability)
    fn class_report(&self)->Result<ClassReport, SystemError>{

        if !self.students.is_empty(){
            let pass_student:Vec<String> = self.students.values().filter(|x| x.score >= 50.0).map(|x| x.name.clone()).collect();

            let failed:Vec<String> = self.students.values().filter(|x| x.score < 50.0).map(|x| x.name.clone()).collect();

            let scholarship:Vec<String> = self.students.values().filter(|x| x.score >= 85.0).map(|x| x.name.clone()).collect();

        Ok(ClassReport{
            pass:pass_student,
            failed:failed,
            scholarship:scholarship,
        })
        }else{
            return Err(SystemError::EmptyDatabase);
        }
        
    }

    
    fn reuseable_status(&self,label:&str, status:Vec<String>, status_comment:&str){
        let mut count = 0;
        match status.is_empty() {
            false => {
            for student in status {
                count +=1;
                 println!("\n{} \n{}", label, student)
                }
                println!("Total No. of students in {}: {}", label, count)
            }
                true => {
                println!("\n{}\n", status_comment)
            }
         }
    }
}