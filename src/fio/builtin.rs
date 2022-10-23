#[cfg(test)]
use crate::fio::common::assert_parse;

use crate::{
    common::FilePosition,
    fio::common::{parse_identifier, Span},
};
use nom::{bytes::complete::tag, combinator::map, sequence::preceded, IResult};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BuiltinType<'a> {
    pub name: &'a str,
    pub position: FilePosition,
}

pub fn parse_builtin(input: Span) -> IResult<Span, BuiltinType> {
    map(preceded(tag("."), parse_identifier), |name| BuiltinType {
        name: &name[..],
        position: input.into(),
    })(input)
}

#[test]
fn test_parse_builtin() {
    assert_parse(
        parse_builtin(Span::new(".Number")),
        BuiltinType {
            name: "Number",
            position: FilePosition { line: 1, column: 1 },
        },
    );
}
