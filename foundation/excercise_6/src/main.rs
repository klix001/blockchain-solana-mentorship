
use std::io;
fn main() {
    // counter();
    // println!("COUNT DOWN");
    // count_down();
    // check_name();
    // even();
    // mul_table();
    // guess_counter();
    // add_number();
    student_portal();

}

// EXCERCISE 1 DAY 6 LOOP COUNTER
fn counter(){
    let mut count=0;

    loop{
        count+=1;
        println!("count: {}",count);

        if count >= 10{
            break
        }
    }
    
}

// EXCERCISE 2 DAY 6 COUNTDOWN
fn count_down(){
     let mut count =10;

     while count>=2{
        count -=1;
        println!("count: {}",count)
     }
}

// EXCERCISE 3 DAY 6 KEEP ASKING NAME UNTIL NOT EMPTY

fn user_input()->String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn check_name(){
    println!("Pls type in your name");
    let mut name = user_input();
    if name.len() <1{
        loop{
            println!("Please entera a valid name");
            name = user_input();
            if name.len()>=1{
                println!("name: {}",name);
                break
            }
        }
    }else{
        println!("name: {}",name);
    }
}

// EXCERCIES 4 PRINT EVEN NUMBER FROM 1-20

fn even(){
    for num in 1..=20{
        if num % 2 ==0 {
            println!("{}",num)
        }
    }
}

// EXCERCISE 5 MULTIPLCATION TABLE

fn mul_table(){
    println!("Enter a number");
    let num:u32 = user_input().parse().expect("failed to read input");

    for i in 1..=12{
        let mut value = num * i;
        println!("5 x {} = {}", i,value);
    }
}

// EXCERCISE 6 GUESS COUNTER 

fn guess_counter(){
    const target:u32 = 7;
    let mut counter = 1;
    println!("guess a number");
    let mut guess:u32 = user_input().parse().expect("invalid user input");
    if guess != 7 {
        loop{
            println!("ops wrong guess! try another number");
            guess = user_input().parse().expect("invalid user input");
            counter +=1;

            if guess == 7 {
                println!("you guessed it at {} tries", counter);
                break
            }
        }
    }else{
        println!("you guessed it at first try");
    }
}

// EXCERCISE 7 ADD SUM
fn add_number(){
    let mut num = 0;
    let mut vector:Vec<i32> = vec!();
   
    while num < 5 {
        println!("pls enter a number");
        let mut number:i32 = user_input().parse().expect("Enter a valid number");
        vector.push(number);
        num +=1;
    }
    let mut total:i32 = vector.iter().sum();
    println!("the total is: {}", total);
}

// MAIN PROJECT STUDENT PORTAL CLI

fn age_validator()->u32{
    println!("Pls enter rour age");
    let age:u32 = user_input().parse().expect("Invalid input");
    if age >=18 {
        println!("your are an Adult");
    }else{
        println!("you are a minor");
    }
    age
}


// GRADE CHECKER
fn grade_checker()->u32{
    println!("Please provide your score");
    let input = user_input().parse().expect("Invalid input");
    if input >=70{
        println!("A")
    }else if input >=60 && input <70{
        println!("B")
    }else if input >=50 && input <60 {
        println!("C")
    }else {
        println!("F")
    }
    input
}

// ADMISSION ELLIGIBILITY CHECKER

fn admission_system(){
    println!("Pls enter your age");
    let age:u32 = user_input().parse().expect("Invalid input");
    println!("Enter your score");
    let grade:f64 = user_input().parse().expect("Invalid input");
    if age >=16 && grade>=85.0{
        println!("Admitted and eligible for a scholarship")
    }else if age >= 16 && grade >=50.0 && grade <85.0{
        println!("admitted")
    }
    else{
        println!("not admitted")
    }
}

fn student_portal(){
     println!("***********MENU***********");
    loop{
        println!("Enter age to Check Age Status");
        println!("Enter grade to Check Grade Result");
        println!("Enter admission to check Admission Eligibility");
        println!("Exit");
        let mut menu = user_input().to_lowercase();
        if menu == "age" {
            let user_age = age_validator();
        }
        if menu == "exit"{
            println!("goodbye friend we are going to miss you!");
            break
        }
        if menu == "grade"{
            let student_grade = grade_checker();
        }
        if menu == "admission"{
            let status_status = admission_system();
        }
    }
}