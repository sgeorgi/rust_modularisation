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

    let mut v = vec![0, 10, 20, 30, 40, 50];
    v.reverse();
    for i in &v { println!("{}", i); };

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    let summary = format!("1: {}, 2: {}", s1, s2);
    println!("Strings are:\n{}", summary);
}
