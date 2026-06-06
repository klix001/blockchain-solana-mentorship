use crate::{
    input_parsing::{read_input}, 
    manager::StudentManager, 
    storage::save, 
    student::Student,
    error::SystemError,
};




pub fn update_score_func(database:&mut StudentManager, name:&str, )->Result<(), SystemError>{
    match database.students.iter_mut().find(|x| x.name.to_lowercase() == name){
            Some(student) => {
                println!("Enter new score");
                let new_score = read_input()?;
                student.score = new_score;
            }
            None => {
                return Err(SystemError::StudentNotFound);
            }
        }
    let _saved = save(&database);
    Ok(())

}

pub fn student_list_func(database: &StudentManager)->Result<Vec<&Student>, SystemError>{
    let student_record:Vec<&Student> = database.students.iter().collect();
    if !student_record.is_empty(){
        return Ok(student_record);
    }
    Err(SystemError::EmptyDatabase)
   }