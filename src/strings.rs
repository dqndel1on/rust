pub fn _run() {
    let mut hello = String::from("Hello ");

    println!("{}", hello);
    println!("{}", hello.len());

    hello.push('w'); //push for single char
    println!("{}", hello);

    hello.push_str("orld!"); //pusn_str for pushing string
    println!("{}", hello)

    //other methods
    //hello.capacity()
    //hello.is_empty;
    //hello.contains("world")
    //hello.replace("World","There")
}
