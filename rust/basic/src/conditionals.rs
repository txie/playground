pub fn run() {
    let age: u8 = 30;
    let check_id: bool = false;
    let knows_person_of_age = true;

    if age >= 21 && check_id || knows_person_of_age {
        println!("bartender: What would you like to drink?");
    }
    else if age < 21 && check_id {
        println!("bartender: Sorry, you have to leave.");
    }
    else {
        println!("bartender: I'll need to check your id.");
    }

    // shorthand if
    let is_of_age = if age >= 21 {true} else {false};
    println!("Is of age: {}", is_of_age);
}