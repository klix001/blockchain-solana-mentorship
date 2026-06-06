use std::path::Path;
use std::fs;
use crate::manager::StudentManager;

const DB_FILE:&str = "students_db.json";

pub fn load_database()->Result<StudentManager, Box<dyn std::error::Error>>{
    if !Path::new(DB_FILE).exists(){
        let student_db:StudentManager = StudentManager::new();
        save(&student_db)?;
        return Ok(student_db);
    }
    let  student_db = fs::read_to_string(&DB_FILE)?;
    let deserilized = serde_json::from_str(&student_db)?;
    Ok(deserilized)
}

pub fn save(database:&StudentManager)->Result<(), Box<dyn std::error::Error>>{
    let serialized = serde_json::to_string_pretty(&database)?;
    fs::write(DB_FILE, serialized)?;
    Ok(())
}