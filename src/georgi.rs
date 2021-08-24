mod constants;

pub fn say_hi() -> &'static str {
    constants::GREETING
}
