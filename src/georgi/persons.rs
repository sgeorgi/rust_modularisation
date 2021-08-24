pub enum Person {
    Sebastian,
    Kalle,
    Eddi,
    Hilda
}

impl Person {
    pub fn age(&self) -> i32 {
        match *self {
            Person::Sebastian => 39,
            Person::Kalle => 6,
            Person::Eddi => 4,
            Person::Hilda => 2,
        }
    }
}
