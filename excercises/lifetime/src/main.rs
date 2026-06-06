fn main() {
    let student1:Student = Student::new(String::from("Kelvin"), 20, 20.0);
    println!("name: {} | age: {} | score: {}",&student1.name, &student1.age, &student1.score);

    println!("name: {}", &student1.name);
    println!("age: {}", &student1.age);
    println!("score: {}", &student1.score);

    let mut student2 = Student::new(String::from("Joel"), 19, 20.0);
    let score = &student2.score;
    println!("score: {}", score);
    update_score(&mut student2);

let vector = student_vector();
    let view = StudentView::new(&vector);
    view.display();

    let longest = longest("kelvin", "joel");
    println!("longest string: {}", longest)
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    let mut max_len:&str="";
    let vector_string:Vec<&str> = vec![x,y];
    for string in vector_string{
        if string.len()> max_len.len(){
            max_len = string;
        }
    }
    return max_len;
}

struct Student{
    name:String,
    age:u32,
    score:f64,
}

impl Student{
    fn new(name:String, age:u32, score:f64)->Self{
        Self{
        name, age, score
        }
    }

    
}

fn update_score(student:&mut Student){
        let new_score = 30.6;
        student.score = new_score
    }

fn student_vector()->Vec<Student>{
    let mut student_vector:Vec<Student> = vec![];
    let student1 = Student::new(String::from("joe"), 22, 67.9);
    let student2 = Student::new(String::from("willie"), 22, 67.9);
    let student3 = Student::new(String::from("john"), 22, 67.9);

    student_vector.push(student1);
    student_vector.push(student2);
    student_vector.push(student3);

    student_vector
}
struct StudentView<'a>{
    students:&'a Vec<Student>
}

impl <'a>StudentView <'a>{
    fn new(students:&'a Vec<Student>)->Self{
        Self { students }
    }

    fn display(&self){
        for student in self.students.iter(){
            println!("{} {} {}", student.name, student.age, student.score);
        }
    }
}