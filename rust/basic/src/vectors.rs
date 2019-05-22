// vectors - resizable arrays
pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    println!("single value: {}", numbers[0]);
    println!("vector length: {}", numbers.len());
    println!("vector occupies {} bytes", std::mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    numbers[2] = 20;
    numbers.push(5);
    numbers.push(6);
    println!("{:?}", numbers);
    numbers.pop();
    println!("{:?}", numbers);

    // loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers vector: {:?}", numbers);
}