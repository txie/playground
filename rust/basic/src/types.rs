// arrays is fixed, vector is various length
// statically typed language. usually infer
pub fn run() {
    // default is "i32"
    let x = 1;
    // default is "f64"
    let y = 2.5;
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    let a = true;
    let b: bool = true;

    let mut z = 5;
    z = 6;
    
    const N: i32 = 5;
    static M: i32 = 5;
    println!("N:{}, M:{}", N, M);

    let (x, y) = (1, 2);
    // Boolean
    let is_active = true;
    let is_greater: bool = 10 > 5;

    println!("{:?}", (x, y, is_active, is_greater));

    let m = plus_one;
    let n = m(5);
    println!("n = {}", n);

    let o: fn(i32) -> i32 = plus_one;
    let p = o(5);
    println!("p = {}", p);

    print_sum(9, 7);

    let a1 = 'a';
    let face = '\u{1F600}';
    println!("{} {}", a1, face);
}

fn print_sum(a: i8, b: i8) {
    println!("sum is {}", a + b);
}

fn plus_one(a: i32) -> i32 {
    a + 1
}