use nom::{
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{char, digit1},
    combinator::map,
    sequence::delimited,
    IResult,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SgmlElement {
    name: String,
    kind: SgmlElementKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SgmlElementKind {
    Aggregate(Vec<SgmlElement>),
    Data(String),
}

/*
pub fn sgml_element(i: &str) -> IResult<&str, SgmlElement> {
    let (i, name) = sgml_open_tag(tag_name)(i)?;
    let (i, _) = whitespace(i)?;

    match terminated(sgml_data1, many_m_n(0, 1, sgml_close_tag(name)))(i) {
        Ok((r, data)) => Ok((r, SgmlElement {
            name: String::from(name),
            kind: SgmlElementKind::Data(String::from(data.trim_end())),
        })),
        _ => {
            let (i, children) = many1(sgml_element)(i)?;
            let (i, _) = whitespace(i)?;
            let (i, _) = sgml_close_tag(name)(i)?;
            Ok((i, SgmlElement {
                name: String::from(name),
                kind: SgmlElementKind::Aggregate(children),
            }))
        },
    }
}
*/

pub fn sgml_open_tag<'a, C: Fn(&'a str) -> IResult<&'a str, &'a str>>(name: C) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    move |i: &str| {
        let (i, _) = char('<')(i)?;
        let (i, n) = name(i)?;
        let (i, _) = char('>')(i)?;
        Ok((i, n))
    }
}

#[allow(clippy::needless_lifetimes)]
fn sgml_close_tag<'a>(name: &'a str) -> impl Fn(&'a str) -> IResult<&'a str, ()> {
    move |i: &str| {
        delimited(tag("</"), tag(name), char('>'))(i).map(|(r, _)| (r, ()))
    }
}

fn sgml_data1(i: &str) -> IResult<&str, &str> {
    // TODO: handle escape codes
    not_open_bracket1(i)
}

fn tag_name(i: &str) -> IResult<&str, &str> {
    take_while1(is_uppercase)(i)
}

fn not_open_bracket1(i: &str) -> IResult<&str, &str> {
    take_while1(is_not_open_bracket)(i)
}

fn whitespace(i: &str) -> IResult<&str, &str> {
    take_while(is_whitespace)(i)
}

fn unsigned_integer(i: &str) -> IResult<&str, u32> {
    map(digit1, |v: &str| v.parse::<u32>().unwrap())(i)
}

fn is_not_open_bracket(chr: char) -> bool {
    (chr as u8) != 0x3C
}

fn is_uppercase(chr: char) -> bool {
    let chr = chr as u8;
    chr >= 0x41 && chr <= 0x5A
}

fn is_whitespace(chr: char) -> bool {
    let chr = chr as u8;
    chr == 0x09 || chr == 0x0A || chr == 0x0D || chr == 0x20
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    fn assert_consumed<'a, C: Fn(&'a str) -> IResult<&'a str, &'a str>>(
        combinator: C,
        input: &'a str,
    ) {
        assert_eq!(combinator(input), Ok(("", input)));
    }

    fn assert_not_consumed<'a, T: 'a, C: Fn(&'a str) -> IResult<&'a str, T>>(
        combinator: C,
        input: &'a str,
    ) {
        assert!(combinator(input).is_err())
    }

    fn assert_consumed_eq<'a, T: 'a, C>(combinator: C, input: &'a str, expected: T)
    where
        T: Debug + PartialEq,
        C: Fn(&'a str) -> IResult<&'a str, T>,
    {
        assert_eq!(combinator(input), Ok(("", expected)));
    }

    /*
    #[test]
    fn sgml_element__empty_element__no_match() {
        assert!(sgml_element("<FOO></FOO>").is_err());
    }

    #[test]
    fn sgml_element__with_data_and_terminated_by_end_tag__match() {
        let expected = SgmlElement {
            name: String::from("FOO"),
            kind: SgmlElementKind::Data(String::from("asdf fdsa")),
        };

        assert_eq!(sgml_element("<FOO>asdf fdsa</FOO><END>"), Ok(("<END>", expected)));
    }

    #[test]
    fn sgml_element__data_with_whitespace_and_terminated_by_end_tag__match() {
        let expected = SgmlElement {
            name: String::from("FOO"),
            kind: SgmlElementKind::Data(String::from("asdf fdsa")),
        };

        assert_eq!(sgml_element("<FOO>  \n \tasdf fdsa  \n\n</FOO><END>"), Ok(("<END>", expected)));
    }

    #[test]
    fn sgml_element__with_data_and_terminated_by_new_tag__match() {
        let expected = SgmlElement {
            name: String::from("FOO"),
            kind: SgmlElementKind::Data(String::from("asdf")),
        };

        assert_eq!(sgml_element("<FOO>asdf<BAR>"), Ok(("<BAR>", expected)));
    }

    #[test]
    fn sgml_element__with_child__match() {
        let expected = SgmlElement {
            name: String::from("FOO"),
            kind: SgmlElementKind::Aggregate(vec![
                SgmlElement {
                    name: String::from("BAR"),
                    kind: SgmlElementKind::Data(String::from("asdf")),
                },
            ]),
        };

        assert_eq!(sgml_element("<FOO><BAR>asdf</BAR></FOO><END>"), Ok(("<END>", expected)));
    }
    */

    #[test]
    fn sgml_open_tag__valid_tag_matching_any__match() {
        assert_eq!(sgml_open_tag(take_while1(is_uppercase))("<FOO>end"), Ok(("end", "FOO")));
    }

    #[test]
    fn sgml_open_tag__lowercase_tag_matching_any__no_match() {
        assert!(sgml_open_tag(take_while1(is_uppercase))("<bad>end").is_err());
    }

    #[test]
    fn sgml_open_tag__valid_tag_matching_specific__match() {
        assert_eq!(sgml_open_tag(tag("FOO"))("<FOO>end"), Ok(("end", "FOO")));
    }

    #[test]
    fn sgml_open_tag__nonmatching_tag_matching_specific__no_match() {
        assert!(sgml_open_tag(tag("FOO"))("<BAR>end").is_err());
    }

    #[test]
    fn sgml_close_tag__valid_tag__match() {
        assert_eq!(sgml_close_tag("FOO")("</FOO>end"), Ok(("end", ())));
    }

    #[test]
    fn sgml_close_tag__different_tag__no_match() {
        assert!(sgml_close_tag("FOO")("</BAR>").is_err());
    }

    #[test]
    fn sgml_data1__immediate_open_bracket__no_match() {
        assert!(sgml_data1("</END>").is_err());
    }

    #[test]
    fn sgml_data1__data_with_whitespace__match() {
        assert_eq!(sgml_data1("a sdf 123  </END>"), Ok(("</END>", "a sdf 123  ")))
    }
    #[test]
    fn tag_name__immediate_end_bracket__no_match() {
        assert!(tag_name(">").is_err());
    }

    #[test]
    fn tag_name__uppercase_name__match() {
        assert_eq!(tag_name("FOO>"), Ok((">", "FOO")))
    }

    #[test]
    fn tag_name__lowercase_name__no_match() {
        assert!(tag_name("foo>").is_err());
    }

    #[test]
    fn not_open_bracket1__immediate_bracket__no_match() {
        assert!(not_open_bracket1("<FOO").is_err());
    }

    #[test]
    fn not_open_bracket1__no_brackets__full_match() {
        let input = " \tasdf1234>!$#$";
        assert_eq!(not_open_bracket1(input), Ok(("", input)));
    }

    #[test]
    fn whitespace__multiple_whitespace_chars__match() {
        assert_consumed(whitespace, " \t \r\n");
    }

    #[test]
    fn whitespace__not_whitespace__empty_match() {
        assert_eq!(whitespace("foo"), Ok(("foo", "")));
    }

    #[test]
    fn unsigned_integer__zero__match() {
        assert_eq!(unsigned_integer("0end"), Ok(("end", 0u32)));
    }

    #[test]
    fn unsigned_integer__large_number__match() {
        assert_eq!(unsigned_integer("1234567890end"), Ok(("end", 1234567890u32)));
    }

    #[test]
    fn unsigned_integer__negative__no_match() {
        let input = "-123";
        assert!(unsigned_integer(input).is_err())
    }

    #[test]
    fn is_not_open_bracket__other_char__returns_true() {
        assert!(is_not_open_bracket('1'));
        assert!(is_not_open_bracket('>'));
    }

    #[test]
    fn is_not_open_bracket__open_bracket__returns_false() {
        assert!(!is_not_open_bracket('<'));
    }

    #[test]
    fn is_uppercase__capital_a__returns_true() {
        assert!(is_uppercase('A'));
    }

    #[test]
    fn is_uppercase__lowercase_a__returns_false() {
        assert!(!is_uppercase('a'));
    }

    #[test]
    fn is_whitespace__whitespace_char__returns_true() {
        let chars = ['\t', '\r', '\n', ' '];
        for c in &chars {
            assert!(is_whitespace(*c));
        }
    }

    #[test]
    fn is_whitespace__letter_char__returns_false() {
        assert!(!is_whitespace('a'));
    }
}
