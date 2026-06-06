use std::collections::HashMap;

pub fn student(){

    let student_vector:Vec<&str> = vec!["Kelvin", "Tolu", "Bimbo", "Titi", "Joel"];
    println!("Names {:?}", student_vector);
    println!("Names {:?}, {:?}", student_vector[0], student_vector[4])

    
}

pub fn hashmap(){
    let scores:HashMap<&str, f64> = HashMap::from([
        ("Kelvin", 90.0),
        ("james", 55.0),
        ("kelechi", 33.0),
        ("joel", 77.0),
    ]);

    for (name, value) in scores {
        print!("\nname: {} score {}", name, value)
    }
}