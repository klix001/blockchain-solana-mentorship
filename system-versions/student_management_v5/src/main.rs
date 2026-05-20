mod model;
mod input_parsing;
mod routing;
mod storage;
mod manager;
mod student;
mod student_behaviour;
use crate::routing::route_user;



fn main() {
    route_user();
}
