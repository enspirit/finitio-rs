#[cfg(test)]
use crate::fio::common::assert_parse;

use crate::{common::FilePosition, fio::common::Span};
use nom::{bytes::complete::tag, combinator::map, sequence::preceded, IResult};
use serde::{Serialize, Deserialize};

use super::common::ws;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AnyType {
    pub position: FilePosition,
}

pub fn parse_any(input: Span) -> IResult<Span, AnyType> {
    map(preceded(ws, tag(".")), |t| AnyType { position: t.into() })(input)
}

#[test]
fn test_parse_any() {
    assert_parse(
        parse_any(Span::new(".")),
        AnyType {
            position: FilePosition { line: 1, column: 1 },
        },
    );
    assert_parse(
        parse_any(Span::new(" .")),
        AnyType {
            position: FilePosition { line: 1, column: 2 },
        },
    );
    assert_parse(
        parse_any(Span::new(" \n  .")),
        AnyType {
            position: FilePosition { line: 2, column: 3 },
        },
    );
}
