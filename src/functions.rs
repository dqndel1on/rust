pub fn _run() {
    _greeting("Hello", "Sanji");

    //binding function values to variables
    let get_sum = _add(2, 3);
    println!("Sum: {}", get_sum);

    //closure
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("C Sum: {}", add_nums(6, 4));
}

fn _greeting(_greet: &str, _name: &str) {
    println!("{} {}, nice to meet you", _greet, _name);
}

fn _add(_a: i32, _b: i32) -> i32 {
    _a + _b
}
