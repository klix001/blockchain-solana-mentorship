use crate::error::SystemError;
use crate::input_parsing::{read_input};




fn parse_age()->Result<u32, SystemError>{
let age = read_input::<u32>()?;
Ok(age)

}