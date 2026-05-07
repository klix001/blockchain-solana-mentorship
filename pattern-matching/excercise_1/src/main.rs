use std::io;
use std::str::FromStr;

fn main() {
    let mut student_record:Vec<Student> = vec!();
    let mut student1:Student = Student::new(String::from("kelvin"), 20, 50.0);
    println!("{:#?}", student1);
    student_record.push(student1);
    menu()
}

fn user_input()->String{
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

#[derive(Debug)]
struct Student{
    name:String,
    age:u32,
    score:f64,
}

impl Student{
    fn new(name:String, age:u32, score:f64)->Self{
        Self{
            name,
            age,
            score,
        }
    }
}

#[derive(Debug)]
enum UserAction{
    AddStudent,
    ViewStudent,
    UpdateStudent,
    ShowAverage,
    Exit,
}

impl FromStr for UserAction{
    type Err = String;
    fn from_str(action:&str)->Result<Self, Self::Err>{
        match action.to_lowercase().as_str(){
            "add_student" => Ok(UserAction::AddStudent),
            "view_student" => Ok(UserAction::ViewStudent),
            "update_student" => Ok(UserAction::UpdateStudent),
            "show_average" => Ok(UserAction::ShowAverage),
            "exit" => Ok(UserAction::Exit),
            _ => Err(format!("{}: This is not a valid action", action)),
        }
    }
}

fn add_student()->Student{
    println!("Enter student name");
    let name:String = user_input();
    println!("Enter student age");
    let age:u32 = user_input().parse().expect("Invalid input");
    println!("Enter student score");
    let score:f64 = user_input().parse().expect("Invalid input");
    let new_student:Student = Student::new(name, age, score);
    new_student
}

fn update_student(student_record:&mut Vec<Student>){
    let mut found:bool = false;
    println!("Provide student name for update");
    let student_name:String = user_input();
    for student in student_record{
        if student.name == student_name {
            found = true;
            println!("Enter new score");
            let new_score:f64 =user_input().parse().expect("Invalid input");
            student.score = new_score;
        }
    }
    if !found {
        println!("{}, is not found in student record", student_name);
    }
}

fn show_average(student_record:&Vec<Student>)->f64{
    let mut total:f64 = 0.0;
    let mut average_score:f64 = 0.0;
    for student in student_record{
        total += student.score;
    }
    if total == 0.0 {
        println!("No valid score in student record");
    }
    else if total > 0.0 {
        let average:f64 = total/student_record.len() as f64;
        average_score+=average;
    }
    average_score
}

fn menu(){
    let mut student_record:Vec<Student> = vec!();
    loop{
    println!("\n******************MENU**********************");
    println!("Enter add_student");
    println!("Enter view_student");
    println!("Enter update_student");
    println!("Enter show_average");
    println!("Enter exit\n");
    let action: UserAction = match user_input().parse(){
        Ok(action) => action,
        Err(_) => {
            println!("Enter a valid action");
            continue;
        }
     };

     match action {
        UserAction::AddStudent => {
            let new_student:Student = add_student();
            student_record.push(new_student);
        },
        UserAction::ViewStudent => {
            println!("{:#?}",student_record);
        },
        UserAction::UpdateStudent => {
            update_student(&mut student_record);
        },
        UserAction::ShowAverage => {
            let average:f64 = show_average(&student_record);
            println!("the average is: {:.2}", average);
        }
        UserAction::Exit => break,
        
     };
    }
}