const MAX_POINTS: u32 = 100_000;

fn shadow1() {
    let x = 5;
    let x = x + 1;
    let x = 13 * 2;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
}
pub fn run() {
    let name = "Brad";

    let mut age = 37;
    println!("My name is {} and I am {}", name, age);

    age = 38;
    println!("My name is {} and I am {}", name, age);

    // define constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple vars
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);

    println!("MAX_POINTS: {}", MAX_POINTS);
    shadow1();
}