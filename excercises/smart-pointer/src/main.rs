use core::fmt;
use std::io;

fn main() {
    let list = Some(Box::new(Node::new(1,
    Some(Box::new(Node::new(2, 
            Some(Box::new(Node::new(3, 
                    None)
                ))
            )))
        ))
    );

if let  Some(node) =  list{
    println!("{}", node);
}

// create multiple student
let student_chain = Some(Box::new(
    StudentChain::new(
        String::from("kelvin"), 
            Some(Box::new(StudentChain::new(
                String::from("joel"), 
                    Some(Box::new(StudentChain::new(
                        String::from("derrick"), 
                            Some(Box::new(StudentChain::new(String::from("kelly"), None)
                        )
                    ))
                ))
            ))
        ))
    ));
if let Some( chain) = student_chain{
    println!("Enter student name");
    let name = user_input();
    println!("{} total: {}", chain, chain.count());
    match chain.find(&name){
        true => println!("name: {}", &name),
        false => println!("user not found"),
    }
}

}


#[derive(Debug)]
struct Node{
    value:u32,
    next:Option<Box<Node>>,
}

impl Node{
    fn new(value:u32, next: Option<Box<Node>>)->Self{
        Self { value, next }
    }
}

impl fmt::Display for Node{
    fn fmt(&self, f:&mut fmt::Formatter<'_>)->fmt::Result{
    match &self.next{
        Some(next) => {
            write!(f, "value: {} point to: {}", self.value, next)
        }
        None => {
            write!(f, "value: {} Empty", self.value)
        }
    }
    }
}


struct StudentChain{
    name:String,
    pointing:Option<Box<StudentChain>>,
    
}

impl StudentChain{
    fn new(name:String, pointing:Option<Box<StudentChain>>)->Self{
        Self { name, pointing }
    }

    fn count(&self)->usize{
        match &self.pointing {
            Some(student) => 1 + student.count(),
            None => 1,
        }
    }

    fn find(&self, name:&str)->bool{
        
        if self.name == name {
            true
        }else{
            match &self.pointing{
                Some(student) => {
                    student.find(name)
                }
                None => false
            }
        }
    }
}

impl fmt::Display for StudentChain{
    fn fmt(&self, f:&mut fmt::Formatter<'_>)->fmt::Result{
        match &self.pointing{
            Some(student)=> {
                write!(f, "{} --> {} ", self.name, student)
            }

            None => write!(f, "{} --> None", self.name)
        }
    }
}


fn user_input()->String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()

}