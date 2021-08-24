pub mod georgi;
pub mod numerology;

#[cfg(test)]
mod tests {
    use crate::georgi::say_hi;
    use crate::numerology::magic_number;

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
        assert_eq!(magic_number(), 42)
    }
}
