extern crate sha1;
extern crate lalrpop_util;

mod spec;
mod parser;

fn main() {
    let spec_str = include_str!("../msg.spec");
    let spec = parser::parse_Specification(spec_str).unwrap();
}
