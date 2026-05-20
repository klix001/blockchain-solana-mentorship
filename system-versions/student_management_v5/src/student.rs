use serde::{Serialize, Deserialize};
use crate::storage::load_database;


#[derive(Debug, Serialize, Deserialize)]
pub struct Student{
    pub name:String,
    pub age:u32,
    pub score:f64,
}

impl Student{
    fn new(name:String, age:u32, score:f64)->Self{
        Self{
            name, age, score
        }
    }
}