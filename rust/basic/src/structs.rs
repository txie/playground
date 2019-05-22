// structs - used to create custom data types
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// tuple struct
struct Color2(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}
impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }
    // name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}
pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };
    c.green = 200;
    println!("Color: {}, {}, {}", c.red, c.green, c.blue);

    let mut c = Color2(255, 0, 0);
    println!("Color: {}, {}, {}", c.0, c.1, c.2);

    let p = Person::new("John", "Doe");
    println!("Person: {} {}", p.first_name, p.last_name);
    println!("Person full name: {}", p.full_name());

    let mut mary = Person::new("Mary", "Doe");
    mary.set_last_name("Williams");
    println!("Person: {}", mary.full_name());

    println!("Person tuple: {:?}", mary.to_tuple());
}