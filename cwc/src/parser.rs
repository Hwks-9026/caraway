// use pest::{iterators::Pair, pratt_parser::{Assoc, Op, PrattParser}};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "caraway.pest"]
pub struct CaraParser;
