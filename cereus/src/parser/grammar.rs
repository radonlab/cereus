use pest::{error::Error, iterators::Pairs, Parser};
use pest_derive::*;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct GrammarParser;

pub fn parse(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    GrammarParser::parse(Rule::program, source)
}
