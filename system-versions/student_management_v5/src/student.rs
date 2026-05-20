use serde::{Serialize, Deserialize};



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