use modular::georgi::persons::Person;
use modular::georgi::say_hi;
use modular::numerology::magic_number;

fn main() {
    let seb = Person::Sebastian;
    let kalle = Person::Kalle;
    let eddi = Person::Eddi;
    let hilda = Person::Hilda;
    println!("{}", say_hi());
    println!("Sebastian is {} years old", seb.age());
    println!("Kalle is {} years old", kalle.age());
    println!("Eddi is {} years old", eddi.age());
    println!("Hilda is {} years old", hilda.age());
    println!("And the answer to everything else is: {}", magic_number());

}
