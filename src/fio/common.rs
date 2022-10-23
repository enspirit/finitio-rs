use nom::{
    bytes::complete::{take_while, take_while1},
    character::complete::char,
    combinator::{map, opt},
    error::{Error, ErrorKind, ParseError},
    sequence::{pair, preceded},
    Err, IResult, Slice
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

/// Adapted from https://github.com/getreu/parse-hyperlinks
pub fn take_until_unbalanced<'a>(
    opening_bracket: char,
    closing_bracket: char,
) -> impl Fn(Span<'a>) -> IResult<Span, Span> {
    move |i: Span<'a>| {
        let mut index = 0;
        let mut bracket_counter = 0;
        while let Some(n) = &i[index..].find(&[opening_bracket, closing_bracket, '\\'][..]) {
            index += n;
            let mut it = i[index..].chars();
            match it.next().unwrap_or_default() {
                c if c == '\\' => {
                    // Skip the escape char `\`.
                    index += '\\'.len_utf8();
                    // Skip also the following char.
                    let c = it.next().unwrap_or_default();
                    index += c.len_utf8();
                }
                c if c == opening_bracket => {
                    bracket_counter += 1;
                    index += opening_bracket.len_utf8();
                }
                c if c == closing_bracket => {
                    // Closing bracket.
                    bracket_counter -= 1;
                    index += closing_bracket.len_utf8();
                }
                // Can not happen.
                _ => unreachable!(),
            };
            // We found the unmatched closing bracket.
            if bracket_counter == -1 {
                // We do not consume it.
                index -= closing_bracket.len_utf8();
                return Ok((i.slice(index..), i.slice(1..index-1)))
            };
        }

        if bracket_counter == 0 {
            return Ok((i.slice(index..), i.slice(1..index-1)))
        } else {
            Err(Err::Error(Error::from_error_kind(i, ErrorKind::TakeUntil)))
        }
    }
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
