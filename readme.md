# PhoneNumberParser

This is a Rust library for parsing and validating Ukrainian phone numbers in the format "+380XXXXXXXXX." It uses the Pest library for parsing.

## Parsing Process

The PhoneNumberParser library is designed to parse and validate Ukrainian phone numbers. It defines a grammar in the `grammar.pest` file, which specifies the format of a valid phone number. The parser then uses this grammar to validate phone numbers provided as input strings.

The parsing process involves the following steps:

1. The input string is processed by the PhoneNumberParser, which checks if it matches the grammar rules.

2. The parser validates that the phone number starts with "+380" and is followed by digits

## Usage

You can use the PhoneNumberParser in your Rust project to validate Ukrainian phone numbers. The library provides a `is_valid_phone_number` function, which returns `true` for valid phone numbers and `false` for invalid ones.

```rust
use phone_number_parser::is_valid_phone_number;

fn main() {
    let phone_numbers = vec!["+380992121211", "+38099232323", "123456789", "+380992121a11"];

    for phone_number in phone_numbers {
        if is_valid_phone_number(phone_number) {
            println!("Valid phone number: {}", phone_number);
        } else {
            println!("Invalid phone number: {}", phone_number);
        }
    }
}
```

You can check different tests in this code block:

```rust
#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_valid_phone_numbers() -> anyhow::Result<()> {
      let pair = PhoneNumberParser::parse(Rule::phoneNumber, "+380992121211")?.next().ok_or_else(|| anyhow!("no pair"))?;

      assert_eq!( pair.as_span().start(), 0 );
      assert_eq!( pair.as_span().end(), 13 );
        Ok(())
    }

    #[test]
    fn test_valid_phone_numbers_fail() -> anyhow::Result<()> {
      let pair = PhoneNumberParser::parse(Rule::phoneNumber, "380992121211");

      assert!( pair.is_err() );
    Ok(())
    }

    #[test]
    fn test_valid_phone_numbers_fail_lenght() -> anyhow::Result<()> {
      let pair = PhoneNumberParser::parse(Rule::phoneNumber, "+380992121")?.next().ok_or_else(|| anyhow!("no pair"))?;

      assert_eq!( pair.as_span().start(), 0 );
      assert_eq!( pair.as_span().end(), 13, "Must be +38 and 10 digits in lenght!" );
        Ok(())
    }

}
