pub fn run() {
    println!("Hello from print.rs file");
    println!("Number: {}", 1);

    // basic formatting
    println!{"{} is from {}", "Brad", "Mass"}

    // positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "hockey");
    // named arguments
    println!("{name} likes to play {activity}", name = "Jack", activity = "Hockey");
    // placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
    // placeholder for debug trait
    println!("{:?}", (12, true, "Brad"));
}