// arrays - fixed list where elements are the same data type
pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    println!("single value: {}", numbers[0]);
    println!("array length: {}", numbers.len());
    println!("array occupies {} bytes", std::mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    numbers[2] = 20;
    println!("{:?}", numbers);

}