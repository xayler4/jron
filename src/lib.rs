#![feature(iter_advance_by)]

mod parser;
mod tokenizer;
mod error;

pub use parser::parse_json;
