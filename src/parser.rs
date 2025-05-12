use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "expr.pest"]
pub struct LispFormatterParser;

#[derive(Debug)]
pub struct Program;

fn parse_pair(pair: Pair<Rule>) -> anyhow::Result<Program> {
    dbg!(pair);
    todo!();
}

pub fn parse(s: &str) -> anyhow::Result<Program> {
    let mut pairs = LispFormatterParser::parse(Rule::program, s)?;

    parse_pair(pairs.next().unwrap())
}
