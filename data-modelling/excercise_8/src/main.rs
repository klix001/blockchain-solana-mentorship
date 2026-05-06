use std::io;
fn main() {
    // let student_record:Vec<Student>=students_vector();
    // let mut student1 = Student::new(String::from("Kelvin"), 20, 80.0);
    // student1.name = String::from("Alice");
    // student1.age = 16;
    // student1.score = 56.0;
    // println!("{:#?}",student1);
    // three_instances();
    // students_vector();
    // top_student();
    // pass_status();
    // search_student(66.9);
    // student_summary();
    student_manager_v2();
    // average(&student_record)
}

fn user_input()->String{
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

// EXCERCISE 1 BASIC STRUCTURE
#[derive(Debug)]
struct Student{
    name:String,
    age:u32,
    score:f64,
}

impl Student{
    fn new(name:String, age:u32, score:f64)->Student{
        Student{
            name, 
            age,
            score,
        }
    }
}

// EXCERCISE 2 CREATE 3 STUDENT INSTANCES AND PRINT ALL THEIR DETAILS
fn three_instances(){
    let student1 = Student::new(String::from("Kelvin"), 20, 96.5);
    let student2 = Student::new(String::from("ALice"), 25, 60.5);
    let student3 = Student::new(String::from("Bob"), 17, 90.5);
println!("{:#?}\n {:#?}\n {:#?}\n", student1, student2, student3)
}

// EXCERCISE 3 CREATE MULTIPLE STUDENT INSTANCES AND STORE IN A VECTOR PRINT ALL THEIR DETAILS
fn students_vector()->Vec<Student>{
    let mut students:Vec<Student> = vec!();
    let student1 = Student::new(String::from("Kelvin"), 20, 96.5);
    let student2 = Student::new(String::from("ALice"), 25, 60.5);
    let student3 = Student::new(String::from("Bob"), 17, 24.5);
    let student4 = Student::new(String::from("Kelly"), 21, 90.6);
    let student5 = Student::new(String::from("Joel"), 28, 49.5);

   students.push(student1);
   students.push(student2);
   students.push(student3);
   students.push(student4);
   students.push(student5);

    for student in &students{
        // println!("{:#?}\n", student)
    }
    students
}

// EXCERCISE 3 CREATE MULTIPLE STUDENT INSTANCES AND STORE IN A VECTOR PRINT ALL THEIR DETAILS
fn top_student(student_record:&Vec<Student>)->f64{
    let mut max_score:f64 = 0.0;
    let student_record:&Vec<Student> = student_record;
    for student in student_record{
        if student.score > max_score{
            max_score = student.score;
        }
        if student.score == max_score{
            println!("The highest scoring student is {}, with a score of {}\n", student.name, student.score);
        }
    }
    max_score
}

// EXCERCISE 5 STUDENT PASS STATUS
fn pass_status(){
   let student_record:Vec<Student> = students_vector();
   for student in &student_record{
    if student.score < 50.0{
        println!("sorry {}, you failed", student.name);
    }else if student.score >= 50.0 {
        println!("congrat! you pass {}", student.name);
    }
   }
}

// EXCERCISE 6 UPDATE STUDENT 
fn search_student(student_records: &mut Vec<Student>, name: String, new_score: f64){
    let mut found: bool = false;

    for student in student_records.iter_mut() {  
        if student.name == name {
            found = true;
            student.score = new_score;         
            println!("{:#?}", student);
        }
    }
    
    if !found {
        println!("User not found");
    }
}

fn average(student_record:&Vec<Student>){
    let students = student_record;
    let mut total:f64 = 0.0; 
    for student in students{
        total+=student.score;
    }
    let average:f64 = total/students.len() as f64;
    println!("{}\n", average);
}
// EXCERCISE STUDENT SUMMARY
fn student_summary(){
    let mut passed:u32 = 0;
    let mut failed:u32= 0;
    let mut total:f64 = 0.0;
    let mut student_record:Vec<Student> = students_vector();
    for student in &mut student_record{
        total+=student.score;
        if student.score > 50.0 {
            passed+=1;
        }else if student.score < 50.0{
            failed+=1;
        }
    }
    let average:f64 = total/student_record.len() as f64;
    println!("{:.2}", average);
    println!("{}", passed);
    println!("{}", failed);

}

// MAIN PROJECT
fn student_manager_v2(){
    let mut student_record:Vec<Student> = vec!();
    loop{
        println!("*****************MENU*******************");
        println!("Enter add to add student");
        println!("Enter view to view students");
        println!("Enter highest to view highest score");
        println!("Enter average to find student average");
        println!("Enter update to update student score");
        println!("Enter exit to exit program");
        let input = user_input();
        if input.to_lowercase() == "add"{
            println!("Pls provide your name");
            let name:String = user_input();
            println!("Pls provide your age");
            let age:u32 = user_input().parse().expect("Invalid input");
            println!("Enter your score");
            let score:f64 = user_input().parse().expect("Invalid input");
            let new_student:Student = Student::new(name, age, score);
            student_record.push(new_student);
            println!("student added");
        }
        else if input.to_lowercase()=="view"{
            println!("{:#?}\n", student_record);
        }
        else if input.to_lowercase() == "highest"{
            let highest:f64 = top_student(&student_record); 
        }
        else if input.to_lowercase() == "average" {
            average(&student_record);
        }
        else if input.to_lowercase() == "update"{
            println!("Enter your name");
            let name:String = user_input();
            println!("Enter new score");
            let new_score:f64 = user_input().parse().expect("Invalid input");
            search_student(&mut student_record, name, new_score);
        }
        else if input.to_lowercase() == "exit"{
            println!("You have exited the program");
            break
        }
    }
}
