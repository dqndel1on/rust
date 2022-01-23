pub fn _run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    numbers.push(6);
    println!("{:?}", numbers);

    numbers.pop();
    println!("{:?}", numbers);
    //other methods same as arrays
    //loop throuch vector values
    // loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("{:?}", numbers);
}
