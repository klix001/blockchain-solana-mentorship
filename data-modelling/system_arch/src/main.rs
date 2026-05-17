use std::io;
use std::str::FromStr;

// EXCERCISE 4 AND MAIN PROJECT IMPLEMENTING ROLE BASED ACCESS FOR STUDENT MANAGEMENT SYSTEM
fn main() {
    role_base();   
    // EXCERCISE 1 TO 3
    // let mut checker = Student{
    //     name:String::from("kelvin"),
    //     age:26,
    //     score:40.5,
    // };
    // checker.pass_fail_checker();
    // checker.summary();
    // checker.check_score();
    // checker.update_score();
    // checker.increment_age();
    // checker.bonus_mark();

}

fn user_input()->String{
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");
    input.trim().to_string()
}

fn student_score()->f64{
        let score: f64 = loop{ match user_input().parse::<f64>(){
        Ok(score) => break score,
        Err(_) => {
            println!("Enter a valid score");
            continue
            }
        };
    };
    score
}  

fn student_age()->u32{
    let age: u32 = loop{ match user_input().parse::<u32>(){
        Ok(age) => break age,
        Err(_) => {
            println!("Enter a valid age ");
            continue
            }
        };
    };
    age
}

//String to UserRoleType
enum UserRole{
        Admin,
        Student,
    }

    impl FromStr for UserRole{
        type Err = String;
        fn from_str(role:&str)->Result<Self, Self::Err>{
            match role.to_lowercase().as_str(){
                "student" =>Ok(UserRole::Student),
                "admin" => Ok(UserRole::Admin),
                _ => Err(format!("Invalid User role {}", role)),
            }
        }
    }

//  ROLE BASED ACCESS ACCESS
fn role_base(){
    println!("indicate role to proceed:");
    let role:UserRole = loop{
        match user_input().parse::<UserRole>(){
            Ok(role) => break role,
            Err(_) => {
                println!("Enter a valid role");
                continue;
            }
        }
    };
    match role {
        UserRole::Student => {Student::student_menu()}
        UserRole::Admin => {Student::admin_menu()},
    }
}

// **EXCERCISE 4 AND MAIN PROJECT STRUCTURE AND LOGIC**

// ADMIN ACTION ENUM
enum AdminAction{
    AddStudent,
    UpdateStudent,
    ListStudent,
    ClassAverage,
    MaxScore,
    Exit,
}

impl FromStr for AdminAction{
    type Err = String;
    fn from_str(action:&str)->Result<Self, Self::Err>{
        match action.to_lowercase().as_str(){
        "add_student" => Ok(AdminAction::AddStudent),
        "update_student" => Ok(AdminAction::UpdateStudent),
        "highest_score" => Ok(AdminAction::MaxScore),
        "list_student" => Ok(AdminAction::ListStudent),
        "class_average" => Ok(AdminAction::ClassAverage),
        "exit" => Ok(AdminAction::Exit),
        _ => Err(format!("Invalid admin action: {}", action)),
        }
    }
}

// STUDENT ACTION ENUM
enum StudentAction{
    PassFailStatus,
    Grade,
    Summary,
    UpdateScore,
    Exit,
}

impl FromStr for StudentAction{
    type Err = String;
    fn from_str(action:&str)->Result<Self, Self::Err>{
        match action.to_lowercase().as_str(){
            "pass_status" => Ok(StudentAction::PassFailStatus),
            "grade" => Ok(StudentAction::Grade),
            "summary" => Ok(StudentAction::Summary),
            "update_score" => Ok(StudentAction::UpdateScore),
            "exit" => Ok(StudentAction::Exit),
            _ => Err(format!("Invalid Operation {}", action)),
        }
    }
}

#[derive(Debug)]
struct Student{
    name:String,
    age:u32,
    score:f64,
}

// student portal and admin portal
impl Student{

    fn new(name:String, age:u32, score:f64) ->Self{
        Self{
            name,
            age,
            score,
        }
    } 

    fn pass_status(&self){
        let pass_threshold = 50.0;
        if self.score >= pass_threshold{
            println!("congrat! {} you passed",self.name);
        }
        else{
            println!("sorry {} you failed", self.name)
        }
    }

    fn search_student(student_record:&mut Vec<Student>)->Option<&String>{
        println!("Enter student name");
        let target:String = user_input();
        for student in student_record{
            if target == student.name{
                println!("Enter new score");
                let new_score:f64 = student_score();
                student.score = new_score;
                return Some(&student.name);

            }
        }
        None
    }

    fn class_average(student_record:&Vec<Student>)->Option<f64>{
        let mut total = 0.0;
        for student in student_record{
            total +=student.score;
        }
        if total == 0.0{
            return None;
        }
        else{
            return Some(total/student_record.len() as f64);
        }
    }

    fn add_student()->Student{
        println!("Enter student name");
        let name:String = user_input();
         println!("Enter student age");
        let age:u32 = student_age();
         println!("Enter student score");
        let score:f64 = student_score();
        Student::new(name, age, score)
    }

    fn update_student_score(&mut self){
        println!("Enter new score");
        let new_score:f64 = student_score();
        self.score = new_score;

    }
    
    fn grade(&self){
        let score = self.score;
        if score >= 70.0{
            println!("your grade is: A")
        }
        else if score >= 60.0{
            println!("your grade is: B")
        }
        else if score >= 50.0 {
            println!("your grade is: C")
        }
        else {
            println!("your grade is: F")
        }
    }
    // STUDENT SECTION
    fn student_menu(){
        println!("Enter your name");
        let name: String = user_input();
        println!("Enter your age");
        let age = student_age();
        println!("Enter your score");
        let score = student_score();
        let mut student1:Student = Student::new(name, age, score);
        loop{
            println!("***************STUDENT MENU****************");
            println!("Enter: pass_status");
            println!("Enter: grade");
            println!("Enter: summary");
            println!("Enter: update_score");
            println!("Enter: exit");

            let action:StudentAction = match user_input().parse(){
                Ok(action) => action,
                Err(_) => {
                    println!("Enter a valid menu");
                    continue;
                }
            };

           match action {
            StudentAction::PassFailStatus => {
                student1.pass_status()
            },
            StudentAction::Grade => {
                student1.grade()
            },
            StudentAction::Summary => {
                println!("{:?}", student1)
            },
            StudentAction::UpdateScore => {
                student1.update_student_score()
            },
            StudentAction::Exit => break,

           } 
        }
    }

    // ADMIN SECTION
fn admin_menu(){
    let mut student_record: Vec<Student> = vec!();
    loop{
        println!("****************ADMIN MENU****************");
        println!("Enter: add_student");
        println!("Enter: update_student");
        println!("Enter: list_student");
        println!("Enter: highest_score");
        println!("Enter: class_average");
        println!("Enter: exit");
        let admin_acton:AdminAction = loop{ match user_input().parse::<AdminAction>(){
            Ok(admin_acton) => break admin_acton,
            Err(_) => {
                println!("Enter a valid menu");
                continue
                }
            }
        }; 
        match admin_acton {
            AdminAction::Exit => break,
            AdminAction::UpdateStudent => {
                match Student::search_student(&mut student_record){
                    Some(name) => println!("score updated for: {}",name),
                    None => println!("User not found"),
                }
            },
            AdminAction::MaxScore => {
                let mut max:f64 = 0.0;
                for student in &student_record{
                    if student.score >max {
                        max=student.score
                    }
                }
                println!("the max is: {}", max);
            },
            AdminAction::ClassAverage =>{
                let average = Student::class_average(&student_record);
                match average {
                    Some(average) => println!("The class average is : {}", average),
                    None => println!("No student in record"),
                }
            },
            AdminAction::ListStudent => {
                for student in &student_record{
                    println!("{:#?}", student)
                }
            },
            AdminAction::AddStudent => {
                let student:Student = Student::add_student();
                student_record.push(student);
            },
            _ => println!("Admin menu not implemented yet"),
        }
    }
}

}

// EXCERCISE 1 TO 3 (ignore the implementation of this in the context of excercise 4 and main project, i will be building those from scrash)
#[derive(Debug)]
struct StudentStruct{
    name:String,
    age:u32,
    score:f64,
}

impl StudentStruct{
    fn pass_fail_checker(&self){
        if self.score >50.0{
            println!("pass")
        }
        else{
            println!("fail")
        }
    }

    fn summary(&self){
        println!("name: {} \nage: {} \nscore: {}", self.name, self.age, self.score);
    }

    fn check_score(&self){
        println!("score: {}", self.score)
    }
    fn update_score(&mut self){
        let new_score = 66.6;
        self.score = new_score;
        println!("The new score is {}", self.score);
    }

    fn increment_age(&mut self){
        let age_increase = 2;
        self.age += age_increase;
        println!("Age has been increased to: {}", self.age)
    }

    fn bonus_mark(&mut self){
        let bonus = 5.0;
        self.score += bonus;
        println!("A bonus of {} was added total score is: {}", bonus, self.score);
    }
}

