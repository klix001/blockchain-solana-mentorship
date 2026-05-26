mod model;
mod input_parsing;
mod routing;
mod teacher;
mod trait_functions;
mod storage;
mod manager;
mod student;
mod traits;
mod student_manager;
use crate::routing::route_user;

fn main() {
    route_user();  
}
