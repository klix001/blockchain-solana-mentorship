use crate::model::Route;
use crate::input_parsing::user_input;
use  crate::manager::StudentManager;
use crate::storage::load_database;


pub fn route_user(){
    let mut manager = load_database_persistent();

        println!("Enter user role: admin | teacher | student");
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
        Route::StudentManager => manager.admin(),
        Route::Teacher => manager.teacher_menu(),
    }

} 

pub fn load_database_persistent()->StudentManager{
    let database  = load_database().unwrap_or_else(|e| {
        eprintln!("Unable to load database {}", e);
        StudentManager::new()
    });
    return database;
}