

fn main() {
    let student:Student = Student { name: String::from("kelvin") };

    println!("{}",student.hello());
    student.goodbye();

    let robot:Robot = Robot {
        id:10
    };

    robot.hello();
    robot.goodbye();
}

struct Student{
    name:String,
}


struct Robot {
    id:u32
}
trait  Greet {
    fn hello(&self)->String;
    fn goodbye(&self)->String;
}

impl Greet for Student{
    
    fn hello(&self)->String {
        format!("hi from {}",self.name)
    }

    fn goodbye(&self)->String {
        format!("bye from {}",self.name)
    }
}
impl Greet for Robot{
    
    fn hello(&self)->String {
        format!("hi from {}",self.id)
    }

    fn goodbye(&self)->String {
        format!("bye from {}",self.id)
    }
}