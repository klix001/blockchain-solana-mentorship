use crate::model::Route;
use crate::input_parsing::user_input;
use  crate::manager::StudentManager;
use crate::student::Student;
use crate::storage::{load_database, save};


pub fn route_user(){
    let mut manager  = load_database().unwrap_or_else(|e| {
        eprintln!("Unable to load database {}", e);
        StudentManager::new()
    });

        println!("Enter user role: student/student manager");
        let user_role:Route = 
        loop{
            match user_input().parse::<Route>(){
                Ok(role) => break role,
                Err(_) => {
                    println!("Enter a valid role");
                    continue;
                }
        }
    };

    match user_role{
        Route::Student => manager.student_menu(),
        Route::StudentManager => manager.student_manager(),
    }

} 