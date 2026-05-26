use serde::{Serialize, Deserialize};
// use crate::traits::{Summary};
use crate::input_parsing::user_input;
use crate::model::StudentAction;



#[derive(Debug, Serialize, Deserialize)]
pub struct Student{
    pub name:String,
    pub age:u32,
    pub score:f64,
}

impl Student{
    pub fn new(name:String, age:u32, score:f64)->Self{
        Self{
            name, age, score
        }
    }
    pub fn menu(&self){
        loop{
            println!("************ student menu **************");
            println!("Enter the following options");
            println!("Option 1: view profile");
            println!("Option 2: pass report");
            println!("Option 3: eligibility");
            println!("Option 4: honourary");
            println!("Option 5: grade");
            println!("Option 6: exit");

            let action:StudentAction = loop {
            match user_input().parse::<StudentAction>(){
                Ok(action) => break action,
                Err(_) => {
                    println!("Enter a valid user action");
                    continue;
                }
            }

        };

        match action{
            StudentAction::ViewProfile => {
                println!("==============={} Profile================", self.name);
                println!("name: {} \nage :{} \nscore: {} ", self.name, self.age, self.score);

            }
            StudentAction::PassReport => {
                if self.score >= 50.0 {
                    println!("Congratulation {} you passed", self.name);
                }else{
                    println!("sorry {} you didnt make the cut", self.name);
                }
            }
            StudentAction::ScholarshipEligibility => {
                if self.score >= 85.0 {
                    println!("congratulation {} you are eligible for a scholarship", self.name);
                }else{
                    println!("Sorry! {} you are not eligible for a scholarship", self.name);
                }
            }
            StudentAction::HonouraryStatus => {
                if self.score >= 85.0 {
                    println!("congratulation! {} you are exceptionally honoured", self.name);
                }else{
                    println!("sorry! {} you didnt make the mark for an honour", self.name);
                }
            }
            StudentAction::Grade => {
                match self.score{
                    70.0..=100.0 => println!("congratulation! {} you got an A", self.name),
                    60.0..=69.0 => println!("good job {} you got a B", self.name),
                    50.0..=59.0 => println!("hi! {} you got a C", self.name),
                    0.0..=49.0 => println!("sorry! {} you scored an F", self.name),
                    _ => println!("Not a valid score"),

                }
            }
            StudentAction::Exit => break,
        }

        };
    }
}

use std::fmt;

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "Name:  {}\nAge:   {}\nScore: {}",
            self.name, self.age, self.score
        )
    }
}


