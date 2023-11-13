use anyhow::{anyhow, Result};
use structopt::StructOpt;
use phone_parser::word_and_number_parser;

#[derive(Debug, StructOpt)]
#[structopt(name = "word_and_number_parser", about = "Parse expressions and dates")]
struct Cli {
    #[structopt(short = "e", long = "expression", help = "Expression to parse")]
    expression: Option<String>,

    #[structopt(short = "d", long = "date", help = "Date to parse")]
    date: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::from_args();

    if let Some(expression) = cli.expression {
        match word_and_number_parser::expression(&expression) {
            Ok(parsed) => println!("Parsed expression: {:?}", parsed),
            Err(err) => eprintln!("Error parsing expression: {}", err),
        }
    }

    if let Some(date) = cli.date {
        match word_and_number_parser::date(&date) {
            Ok(parsed) => println!("Parsed date: {:?}", parsed),
            Err(err) => eprintln!("Error parsing date: {}", err),
        }
    }

    Ok(())
}
