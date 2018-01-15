extern crate ess;

fn main() {
    let schema = include_str!("../msg.scm");
    let (sexp, err) = ess::parser::parse(schema);
    println!("{:?}: {:?}", err, sexp);
}
