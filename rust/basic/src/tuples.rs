// max 12 elements
// Tuples are also immutable by default and even with mut, 
// its element count cannot be changed. Also, if you want to change an element’s value, 
// the new value should have the same data type of previous value.
pub fn run() {
    let person: (&str, &str, i8) = ("Brad", "Mass", 37);
    println!("{} is from {} and is {}", person.0, person.1, person.2);

    let a = (1, 1.4, true, 'a', "Hello World");
    let b: (i32, f64) = (1, 1.5);
    let (c, d) = b;
    let (e, _, _, _, f) = a;
    let g = (0, );
    let h = (b, (2, 4), 5);
    println!("c: {:?}, a: {:?}, g: {:?}, h: {:?}, e: {:?}, f: {:?}", c, a, g, h, e, f);
    // println!("c: {}")
}