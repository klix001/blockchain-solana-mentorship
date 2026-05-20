use std::io;


pub fn user_input()->String{
    let mut input:String = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
    input.trim().to_string()
}

pub fn read_u32()->u32{
    let integer:u32 = loop{
        match user_input().parse::<u32>(){
            Ok(value) => break value,
            Err(_) => {
                println!("Enter a valid whole number");
                continue;
            }
        }
    };
    integer
}

pub fn read_f64()->f64{
    let decimal:f64 = loop{
        match user_input().parse::<f64>(){
            Ok(value) => break value,
            Err(_) => {
                println!("Enter a valid 2 decimal place number");
                continue;
            }
        }
    };
    decimal
}