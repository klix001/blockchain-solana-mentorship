mod hashmap;
use crate::hashmap::{student, hashmap};

fn main() {

    // let num1 =1;
    // let num2 =num1; //num1 still exist even after it has been copied to num2

    // let string = String::from("Kalrvin");
    // let string2 = string; //string no longer exist because it has lost ownership to string2


    // let book1 = Book::new(String::from("clash of titans"), String::from("Kelvin"));
    // let book2 = Book::new(String::from("clash of titans"), String::from("Kelvin"));

    // let config = Configuration;
    // let config2 = Configuration;

    // if config==config2 {
    //     println!("The config are the same");
    // }
    // let default = Configuration::default();
    // println!("{:?}", default);

    // let config3 = config2.clone();

    // let book3 = Book::default();
    // if book1 == book2 {
    //     println!("Default {:?}" , book3);
    // }else{
    //     println!("The books are diffirent");
    // }

    // student();
    hashmap();

}

#[derive(Debug, PartialEq, Clone, Default)]

struct Book{
title:String,
author:String,
}

impl Book{
    fn new(title:String, author:String)->Self{
        Self { title, author}
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
struct Configuration;
