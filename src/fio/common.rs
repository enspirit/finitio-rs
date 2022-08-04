use nom::{
    bytes::complete::{take_while, take_while1},
    character::complete::char,
    combinator::{map, opt},
    sequence::{pair, preceded},
    IResult,
};
use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a str>;

const WHITESPACE: &str = " \t\r\n";

pub fn ws(input: Span) -> IResult<Span, Span> {
    take_while(move |c| WHITESPACE.contains(c))(input)
}

pub fn ws1(input: Span) -> IResult<Span, Span> {
    take_while1(move |c| WHITESPACE.contains(c))(input)
}

pub fn trailing_comma(input: Span) -> IResult<Span, Option<char>> {
    opt(preceded(ws, char(',')))(input)
}

const IDENTIFIER_EXTRA: &str = "._";
pub fn parse_identifier(input: Span) -> IResult<Span, String> {
    map(
        pair(
            take_while1(move |c: char| c.is_ascii_alphabetic()),
            take_while(move |c: char| c.is_ascii_alphanumeric() || IDENTIFIER_EXTRA.contains(c)),
        ),
        |t| format!("{}{}", t.0, t.1),
    )(input)
}

#[cfg(test)]
pub(crate) fn assert_parse<'a, T: std::fmt::Debug + PartialEq>(
    output: IResult<LocatedSpan<&'a str>, T>,
    expected_value: T,
) {
    assert!(output.is_ok(), "{:?}", output);
    let output = output.unwrap();
    assert_eq!(output.0.fragment(), &"");
    assert_eq!(output.1, expected_value);
}
