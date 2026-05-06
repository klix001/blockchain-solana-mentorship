use std::io;

fn main() {
    println!("please type in your name");
    let name = user_input();
    // age_validator(age);
    println!("pls enter your age");
    let age:u32 = user_input().trim().parse().expect("Failed to read input");
    println!("pls type in your grade");
    let grade:f64 = user_input().trim().parse().expect("Failed to read input");
    // grade_checker(input);
    admission_system(&name, age, grade);


}

// AGE VALIDATOR
fn age_validator(age:u32){
    if age >=18 {
        println!("your are an Adult");
    }else{
        println!("you are a minor");
    }
}

fn user_input()->String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

// GRADE CHECKER

fn grade_checker(input:u32){
    if input >=70{
        println!("A")
    }else if input >=60 && input <70{
        println!("B")
    }else if input >=50 && input <60 {
        println!("C")
    }else {
        println!("F")
    }
}

// ADMISSION ELLIGIBILITY CHECKER

fn admission_system(name:&str, age:u32, grade:f64){
    if age >=16 && grade>=85.0{
        println!("eligible for admission")
    }else if age >= 16 && grade >=50.0 && grade <85.0{
        println!("admitted")
    }
    else{
        println!("not admitted")
    }
}