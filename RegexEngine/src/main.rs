mod automaton;
mod macros;
mod regex_parser;
mod state_generator;
mod tree;
mod type_defs;

#[cfg(test)]
mod tests;

use crate::regex_parser::RegexParser;

fn main() {
    let parser = RegexParser::from("(a|b)*");
    println!("{:?} {:?}", parser.parse("ababab"), Some(0));
    println!("{:?} {:?}", parser.parse("aaaaaa"), Some(0));
    println!("{:?} {:?}", parser.parse("bbbbbb"), Some(0));
    println!("{:?} {:?}", parser.parse("c"), Some(0)); // matches empty string
}
