pub fn run() {
    // primitive string
    let hello = "Hello";
    // growable, heap-allocated data structure      
    let mut m_hello = String::from("Hello ");
    println!("Length: {}", hello.len());
    m_hello.push('W');
    m_hello.push_str("orld");

    println!("Capacity: {}", m_hello.capacity());
    println!("Is empty: {}", m_hello.is_empty());
    println!("Contains 'World' {}", m_hello.contains("World"));

    // replace
    println!("Replace: {}", m_hello.replace("World", "There"));

    // loop throu string by whitespace
    for word in m_hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("s = {}", s);
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", m_hello);
}