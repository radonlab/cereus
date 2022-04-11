use pest_derive::Parser as PestParser;

#[derive(PestParser)]
#[grammar = "parser/grammar.pest"]
struct Parser;
