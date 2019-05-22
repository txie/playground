pub fn run() {
    greeting("Hello", "Jane");
    // bind function values to variables
    let get_sum = add(5, 4);
    println!("Sum: {}", get_sum);

    // closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3, 3));
    println!("Squre:{}", get_square_value(4));

    let square = |x: i32| -> i32 {
        x * x
    };
    println!("{}", square(3))
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn get_square_value(x: i32) -> i32 {
    x * x
}