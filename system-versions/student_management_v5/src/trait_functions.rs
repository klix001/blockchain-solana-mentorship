use crate::{input_parsing::{read_f64, user_input}, manager::StudentManager, storage::save, student::Student};




pub fn update_score_func(database:&mut StudentManager){
    println!("Enter student name");
    let name = user_input();
    let _student = match database.students.iter_mut().find(|x| x.name.to_lowercase() == name){
            Some(student) => {
                println!("Enter new score");
                let new_score = read_f64();
                student.score = new_score;
                student.name.clone()
            }
            None => {
                println!("student not found");
                return;
            }
        };
        let _saved = save(&database);
        println!("score Successfully updated for {}", name);
}

pub fn student_list_func(database: &StudentManager){
    let mut count = 0;
    let student_record:Vec<&Student> = database.students.iter().collect();
    if !student_record.is_empty(){
        for student in student_record{
            count+=1;
            println!("===============Student {}================", count);
            println!("name: {} \nage: {} \nscore{}", student.name, student.age, student.score);
        }
    }else{
        println!("Student not found in database")
    }
    println!("Total number of student: {}", count);
   }