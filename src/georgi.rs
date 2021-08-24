mod constants;
pub mod persons;


pub fn say_hi() -> &'static str {
    constants::GREETING
}
