pub mod georgi;

#[cfg(test)]
mod tests {
    use crate::georgi::say_hi;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn say_hi_works() {
        assert_eq!(say_hi(), "Hi, this is Georgi")
    }
}
