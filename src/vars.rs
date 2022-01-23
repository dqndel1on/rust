//Variables

#[allow(dead_code)]
pub fn _run() {
    //let
    let _name = "Sanji";
    //mut makes the variable mutable (changable )
    let mut _age = 16;

    _age = 17;
    println!("My name is {} and I am {}", _name, _age);

    const ID: i32 = 001;
    println!("My ID is {}", ID);

    let (my_name, my_age) = (_name, _age);
    println!("{} is {}", my_name, my_age);
}
