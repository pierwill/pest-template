use pest::Parser;

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "renameme.pest"]
pub struct RenamemeParser;

fn main() {

    let input = "(Hello (world))";
    
    let p = RenamemeParser::parse(Rule::list, &input)
        .expect("Parsing error")
        .next()
        .unwrap();

    println!("{:#?}", p);
}
