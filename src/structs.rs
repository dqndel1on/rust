//struct Color {
//    red: u8,
//    green: u8,
//    blue: u8,
//}

//Tuple struct
//struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    //Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
}
pub fn _run() {
    //let mut c = Color {
    //    red: 255,
    //    green: 0,
    //    blue: 0,
    //};
    //println!("Color: {} {} {}", c.red, c.green, c.blue);

    //let c = Color(255, 0, 0);
    //println!("Color: {} {} {}", c.0, c.1, c.2);

    let p = Person::new("Sanji", "Pun");
    println!("Person {} {}", p.first_name, p.last_name);
}
