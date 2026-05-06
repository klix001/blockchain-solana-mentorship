
use std::io;

fn main() {
    // arrays();
    // sum_score();
    // find_max();
    // even_number();
    // number();
    // search_value()
    // name_list();
    student_management();
}

// USER INPUT 
fn user_input()->String{
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}


// EXCERCISE 7 ARRAY
const ARRAY :[u32; 5]= [20,50,60,70,40];
fn arrays(){
    let scores: [u32; 5]= ARRAY;
    println!("{:?}", scores);
    println!("{}", scores[0]);
    println!("{}", scores[4]);
    println!("{}", scores.len());

}

// SUM SCORE
fn sum_score(){
    let scores:[u32; 5] = ARRAY;
    let mut total:u32 = 0;
    for i in scores{
        total +=i;
    }
    let average:f64 = total as f64/scores.len() as f64;
    println!("{}", average)
}

// FIND MAX
fn find_max(){
    let mut max:u32 = 0;
    let scores:[u32;5] = ARRAY;
    for highest in scores {
        if highest >max {
            max=highest;
        }
    }
    println!("The max is: {}",max);
}

// STORE EVEN NUMBER
fn even_number(){
    let mut even: Vec<u32> = vec!();
    for number in 1..=20{
        if number % 2 == 0{
            even.push(number);
        }
    }
    println!("{:?}", even);
}

// ASK USER FOR NUMBER
fn number(){
    let mut count =0;
    let mut numbers:Vec<u32> = vec!();
    let mut max:u32 =0;
    loop{
        println!("please Enter a number");
        let input:u32 = user_input().parse().expect("invalid input");
        numbers.push(input);
        count +=1;

        if count >=5 {
            break
        }
    }
    let total:u32 = numbers.iter().sum();
    println!("{:?}", numbers);
    println!("{}", total);
    for highest in numbers {
        if highest > max {
            max = highest;
        }
    }
    println!("{}", max);
}

// EXCERCISE 6 SEARCH VALUE
fn search_value(){
    println!("Search a target");
    let mut found:bool = false;
    let numbers:[u32;10] = [12,1,2,7,9,10,13,5,6,8];
    let input:u32 = user_input().parse().expect("Invalid input");

    for target in numbers{
        if input == target {
            found = true;
            break
        }
    }
    if found {
        println!("Target found: {}", input);
    }else{
        println!("Target not found")
    }
}

// EXCERCISE 7 NAME LIST
fn name_list(){
    println!("Please enter three names");
    let mut count = 0;
    let mut names:Vec<String> = vec!();
    loop{
        let input = user_input();
        names.push(input);
        count+=1;

        if count >= 3{
            break
        }
    }
println!("the names are : {}, {}, {}", names[0], names[1], names[2]);

}

fn student_management(){
    let mut student_name:Vec<String> =vec!();
    let mut student_score:Vec<f64> =vec!();
    let mut max:f64 = 0.0;

    loop{
        println!("**************MENU*******************");
        println!("enter name: to add name");
        println!("enter score: to add score");
        println!("enter view: to view students");
        println!("enter average: to show average score");
        println!("enter highest: to show highest score");
        println!("enter exit to: exit program");
        let input = user_input().to_lowercase();

        if input =="name"{
            println!("Please provide your name");
            let name = user_input();
            student_name.push(name);
        }
        else if input == "score"{
            println!("Enter your score");
            let score:f64 = user_input().trim().parse().expect("Invalid input");
            &student_score.push(score);
        }
        else if input == "view"{
            println!("Student name: {:?}\n", student_name)
        }
        else if input =="highest" {
              let mut max:f64 = 0.0;
        for highest in &student_score {
        if *highest >max {
            max=*highest;
        }
    }
    println!("The max is: {}",max);
        }else if input == "exit"{
              println!("you have exited the program");
            break
        }else if input =="average"{
            let average:f64 = student_score.iter().sum::<f64>()/student_score.len()as f64;
            println!("The average is : {}\n", average);
        }
        }
        
    }
    
