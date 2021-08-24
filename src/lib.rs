pub mod georgi;
mod numerology;

#[cfg(test)]
mod tests {
    use crate::georgi::say_hi;
    use crate::numerology;
    use crate::numerology::age;
    use crate::numerology::age::Person;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn say_hi_works() {
        assert_eq!(say_hi(), "Hi, this is Georgi")
    }

    #[test]
    fn magic_number_works() {
        assert_eq!(numerology::magic_number(), 42)
    }

    #[test]
    fn age_works() {
        let sebastian: Person = Person { age: 39 };
        assert_eq!(age::age_of(&sebastian), 39)
    }
}
