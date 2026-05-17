fn main() {
    // iterator_excercises();
    student_filter();
}

// EXCERCISE ONE VECTOR CONTAINING EVERY ELEMENT +5
fn iterator_excercises(){
    let vector = vec![45,72,30,90,99,78];
    let result:Vec<u32> = vector.iter().map(|x| x+5).collect();
    println!("{:?}",result);

    let greater:Vec<&u32> = vector.iter().filter(|x| **x>50).collect();
    println!("{:?}", greater);

    let string:Vec<String> = vector.iter().map(|x| x.to_string()).collect();
    println!("{:?}", string);
}

#[derive(Debug)]
struct Student{
    name:String,
    score:u32,
}

impl Student{
    fn new(name:String, score:u32)->Self{
        Self{
            name, score,
        }
    }

    
}













// EXCERCISE 2 
fn student_filter(){
    let mut student_record:Vec<Student> = vec![];
    let student1:Student = Student::new(String::from("kelvin"), 30); 
    let student2:Student = Student::new(String::from("alice"), 50); 
    let student3:Student = Student::new(String::from("joe"), 40); 
    let student4:Student = Student::new(String::from("kevin"), 80); 
    let student5:Student = Student::new(String::from("kelly"), 80); 
    let student6:Student = Student::new(String::from("james"), 90); 

    student_record.push(student1);
    student_record.push(student2);
    student_record.push(student3);
    student_record.push(student4);
    student_record.push(student5);
    student_record.push(student6);

    let filter:Vec<&Student> = student_record.iter().filter(|x| x.score > 70).collect();
    let name_vector:Vec<&str> = student_record.iter().map(|x| x.name.as_str()).collect();
    let low_grade: Vec<&str> = student_record.iter().filter(|x| x.score<40).map(|x| x.name.as_str()).collect();
    let first_low_grade:&str = low_grade[0];


    println!("{:#?}", first_low_grade);
}
