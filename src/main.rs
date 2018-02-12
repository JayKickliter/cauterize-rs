extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::{Parser, Error as PestError};

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("grammar.pest");

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct SpecParser;

fn main() {
    let spec_str = include_str!("../msg.spec");
    let pairs = match SpecParser::parse(Rule::spec, spec_str) {
        Ok(p) => p,
        Err(e) => {
            match e {
                PestError::ParsingError { positives, pos, .. } => {
                    let (line_no, col_no) = pos.line_col();
                    println!("Invalid Cauterize specification syntax at line {}, col {}",
                             line_no,
                             col_no);
                    println!("{:?}", positives);

                    ::std::process::exit(1);
                }
                _ => unreachable!(),
            }
        }
    };
    for pair in pairs {
        for inner_pair in pair.into_inner() {
            println!("Rule: {:?}", inner_pair.as_rule());
            println!("      {}", inner_pair.into_inner());
        }
    }
}
