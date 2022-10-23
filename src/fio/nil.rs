#[cfg(test)]
use crate::fio::common::assert_parse;

use crate::{common::FilePosition, fio::common::Span};
use nom::{bytes::complete::tag, combinator::map, sequence::preceded, IResult};
use serde::{Serialize, Deserialize};

use super::common::ws;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct NilType {
    pub position: FilePosition,
}

pub fn parse_nil(input: Span) -> IResult<Span, NilType> {
    map(preceded(ws, tag("Nil")), |t| NilType { position: t.into() })(input)
}

#[test]
fn test_parse_nil() {
    assert_parse(
        parse_nil(Span::new("Nil")),
        NilType {
            position: FilePosition { line: 1, column: 1 },
        },
    );
    assert_parse(
        parse_nil(Span::new(" Nil")),
        NilType {
            position: FilePosition { line: 1, column: 2 },
        },
    );
    assert_parse(
        parse_nil(Span::new(" \n  Nil")),
        NilType {
            position: FilePosition { line: 2, column: 3 },
        },
    );
}
