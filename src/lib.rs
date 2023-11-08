pub use pest_derive::Parser;
pub use pest::Parser;
use anyhow::anyhow;

  #[derive(Parser)]
  #[grammar = "grammar.pest"]
  struct PhoneNumberParser;


peg::parser! {
        pub grammar phone_number_parser() for str {
            rule digit() -> char
                = ['0'..='9']
    
            pub rule digits() -> &'input str
                = s:$(['0'..='9']+) { s }
    
            pub rule phone_number() -> bool
                = "+" "38" digits() { true }
                / { false }
        }
}



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
