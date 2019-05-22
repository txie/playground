// arrays - fixed list where elements are the same data type
// Arrays are immutable by default and even with mut, its element count cannot be changed.
pub fn run() {
    println!("===== arrays =====");

    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    println!("single value: {}", numbers[0]);
    println!("array length: {}", numbers.len());
    println!("array occupies {} bytes", std::mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    numbers[2] = 20;
    println!("{:?}", numbers);

    let mut b: [i32; 3] = [1, 2, 3];
    let e = ["my value"; 3];
    println!("{:?}", b);
    println!("{:#?}", e);

    let mut a: [i32; 4] = [1, 2, 3, 4];
    let b: &[i32] = &a; // slice whole array
    println!("> {:?}, {:?}", a, b);
    // FIXME a[1] = 20;
    println!("> {:?}, {:?}", a, b);
}