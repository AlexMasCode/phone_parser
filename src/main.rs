pub use anyhow::anyhow;
use phone_parser::*;

pub fn main() -> anyhow::Result<()> {

    let phone_numbers = vec!["+380992121211", "+380993231211", "123456789"];

    for phone_number in phone_numbers {
        if phone_number_parser::phone_number(phone_number).is_ok() {
            println!("Valid phone number: {}", phone_number);
        } else {
            println!("Invalid phone number: {}", phone_number);
        }
    }
    Ok(())
}