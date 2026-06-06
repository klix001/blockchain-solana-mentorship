use std::io;
use std::str::FromStr;

use crate::error::SystemError;


pub fn user_input()->String{
    let mut input:String = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
    input.trim().to_string()
}

pub fn read_input<T: FromStr>()->Result<T, SystemError>
    where SystemError: From<<T as FromStr>::Err>
{
    let value = user_input().parse::<T>()?; 
    Ok(value)
}
