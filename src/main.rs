#[macro_use]
extern crate combine;

use combine::char::{char, letter, spaces};
use combine::{between, many1, parser, sep_by, Parser};
use combine::primitives::{State, Stream, ParseResult};

#[derive(Debug, PartialEq)]
pub struct KeyValuePair {
    key: String,
    value: JsonElement,
}

#[derive(Debug, PartialEq)]
pub enum JsonElement {
    Number(i64),
    Text(String),
    Array(Vec<JsonElement>),
    Hash(Vec<KeyValuePair>)
}

parser! {
    fn expr[I]()(I) -> JsonElement where [I: Stream<Item=char>] {
        let word = many1(letter());

        let mlet  = || many1(letter());
        let lchar = |c| char(c).skip(spaces());

        let expr_comma = sep_by(expr(), lchar(','));
        let array = between(lchar('['), lchar(']'), expr_comma);

        let key_value = || (
            mlet(),
            lchar(':'),
            expr()
        ).map(|t| KeyValuePair { key: t.0, value: t.2 });
        let key_value_comma = sep_by(key_value(), lchar(','));
        let hash = between(lchar('{'), lchar('}'), key_value_comma);

        word.map(JsonElement::Text)
            .or(array.map(JsonElement::Array))
            .or(hash.map(JsonElement::Hash))
            .skip(spaces())
    }
}

fn main() {
    let result = expr().parse("{a: [a, b, {a: Y, b: Z}]}");

    println!("{:?}", result);
    println!("Hello, world!");
}
