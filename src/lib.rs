pub use pest_derive::Parser;
pub use pest::Parser;
use anyhow::{anyhow, Result};
use structopt::StructOpt;
use std::convert::Infallible;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct MyParser;

peg::parser! {
  pub grammar word_and_number_parser() for str {
    rule letter() -> char
            = ['a'..='z' | 'A'..='Z']

        rule digit() -> char
            = ['0'..='9']

        pub rule word() -> &'input str
            = s:$(['a'..='z' | 'A'..='Z']+) { s }

        pub rule number() -> &'input str
            = s:$(['0'..='9']+) { s }

        pub rule word_or_number() -> &'input str
            = s:$(['a'..='z' | 'A'..='Z']+ / ['0'..='9']+) { s }

      pub rule date() -> Date
          = y:number() "-" m:number() "-" d:number() {
              Date {
                  year: y.to_string(),
                  month: m.to_string(),
                  day: d.to_string(),
                  
              }
          }

      
  }
}

#[derive(Debug)]
pub struct Date {
  pub day: String,
  pub month: String,
  pub year: String,
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_valid_date() {
        let parsed_date = word_and_number_parser::date("2022-11-13").unwrap();

        assert_eq!(parsed_date.day, "13");
        assert_eq!(parsed_date.month, "11");
        assert_eq!(parsed_date.year, "2022");
    }

    #[test]
    fn test_parse_invalid_date() {
        assert!(word_and_number_parser::date("2022-11-13-15").is_err());
    }

    #[test]
    fn test_parse_valid_date2() {
        assert!(word_and_number_parser::date("2022-11-13").is_ok());
    }

    #[test]
    fn test_valid_words() {
        assert_eq!(word_and_number_parser::word("hello"), Ok("hello"));
        assert_eq!(word_and_number_parser::word("World"), Ok("World"));
    }

    #[test]
    fn test_valid_numbers() {
        assert_eq!(word_and_number_parser::number("123"), Ok("123"));
        assert_eq!(word_and_number_parser::number("42"), Ok("42"));
    }

    #[test]
    fn test_valid_word_or_number() {
        assert_eq!(word_and_number_parser::word_or_number("hello"), Ok("hello"));
        assert_eq!(word_and_number_parser::word_or_number("123"), Ok("123"));
    }

    #[test]
    fn test_invalid_word_or_number() {
        assert!(word_and_number_parser::word_or_number("").is_err());
        assert!(word_and_number_parser::word_or_number("!@#").is_err());
    }
}

// #[cfg(test)]
// mod tests {

//   use super::*;


//     #[test]
//     fn test_valid_phone_numbers() -> anyhow::Result<()> {
//       let pair = PhoneNumberParser::parse(Rule::phoneNumber, "+380992121211")?.next().ok_or_else(|| anyhow!("no pair"))?;

//       assert_eq!( pair.as_span().start(), 0 );
//       assert_eq!( pair.as_span().end(), 13 );
//         Ok(())
//     }

//     #[test]
//     fn test_valid_phone_numbers_fail() -> anyhow::Result<()> {
//       let pair = PhoneNumberParser::parse(Rule::phoneNumber, "380992121211");

//       assert!( pair.is_err() );
//     Ok(())
//     }

//     #[test]
//     fn test_valid_phone_numbers_fail_lenght() -> anyhow::Result<()> {
//       let pair = PhoneNumberParser::parse(Rule::phoneNumber, "+380992121")?.next().ok_or_else(|| anyhow!("no pair"))?;

//       assert_eq!( pair.as_span().start(), 0 );
//       assert_eq!( pair.as_span().end(), 13, "Must be +38 and 10 digits in lenght!" );
//         Ok(())
//     }

// }
