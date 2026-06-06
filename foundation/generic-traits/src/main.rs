

fn main() {
    let vector:u32= 200;
    display(&vector);

    let record_int= Record { value: 300 };
 
    let value = record_int.get();
    println!("{}", value)
}

struct Teacher{
    teacher_id:String,
    course:String,
    department:String,
}

fn display<T: std::fmt::Display>(item: &T){
    println!("{}",item);
}


struct Record<T>{
    value:T
}

impl<T> Record<T>{
    fn get(&self)->&T{
        &self.value
    }
}