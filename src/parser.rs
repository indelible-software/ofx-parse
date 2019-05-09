use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, digit1, line_ending, not_line_ending},
    combinator::map,
    multi::separated_list,
    sequence::pair,
    IResult,
    ParseError,
};

use crate::model::{Charset, Data, Encoding, HeaderEntry, Ofx, Security};

pub fn ofx(i: &str) -> IResult<&str, Ofx> {
    map(v1_header, |hs| Ofx { headers: hs })(i)
}

fn v1_header(i: &str) -> IResult<&str, Vec<HeaderEntry>> {
    separated_list(line_ending, v1_header_entry)(i)
}

fn v1_header_entry(i: &str) -> IResult<&str, HeaderEntry> {
    pair(alphanumeric1, char(':'))(i).and_then(|(i2, (e, _))| header_value(e, i2))
}

fn header_value<'a>(element: &str, i: &'a str) -> IResult<&'a str, HeaderEntry> {
    match element {
        "CHARSET" => map(header_charset_value, HeaderEntry::Charset)(i),
        "COMPRESSION" => map(not_line_ending, |_| HeaderEntry::Compression)(i),
        "DATA" => map(header_data_value, HeaderEntry::Data)(i),
        "ENCODING" => map(header_encoding_value, HeaderEntry::Encoding)(i),
        "NEWFILEUID" => map(header_file_id_value, HeaderEntry::NewFileUid)(i),
        "OFXHEADER" => map(unsigned_integer, HeaderEntry::OfxHeader)(i),
        "OLDFILEUID" => map(header_file_id_value, HeaderEntry::OldFileUid)(i),
        "SECURITY" => map(header_security_value, HeaderEntry::Security)(i),
        "VERSION" => map(unsigned_integer, HeaderEntry::Version)(i),
        _ => map(not_line_ending, |v| HeaderEntry::Unknown(String::from(v)))(i),
    }
}

fn header_charset_value(i: &str) -> IResult<&str, Option<Charset>> {
    alt((
        map(none, |_| None),
        map(tag("ISO-8859-1"), |_| Some(Charset::Iso88591)),
        map(tag("1252"), |_| Some(Charset::Windows1252)),
    ))(i)
}

fn header_data_value(i: &str) -> IResult<&str, Data> {
    alt((
        map(tag("OFXSGML"), |_| Data::OfxSgml),
        map(not_line_ending, |v| Data::Other(String::from(v))),
    ))(i)
}

fn header_encoding_value(i: &str) -> IResult<&str, Encoding> {
    alt((
        map(tag("USASCII"), |_| Encoding::UsAscii),
        map(tag("UTF-8"), |_| Encoding::Utf8),
    ))(i)
}

fn header_file_id_value(i: &str) -> IResult<&str, Option<String>> {
    alt((
        map(none, |_| None),
        map(not_line_ending, |v| Some(String::from(v))),
    ))(i)
}

fn header_security_value(i: &str) -> IResult<&str, Option<Security>> {
    alt((
        map(none, |_| None),
        map(tag("TYPE1"), |_| Some(Security::Type1)),
    ))(i)
}

fn sgml_tag<O, F>(tag_name: F, i: &str) -> IResult<&str, O> where
    F: Fn(I) -> IResult<&str, O>,
{
    unimplemented!()
}

fn none(i: &str) -> IResult<&str, &str> {
    tag("NONE")(i)
}

fn unsigned_integer(i: &str) -> IResult<&str, u32> {
    map(digit1, |v: &str| v.parse::<u32>().unwrap())(i)
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

    #[test]
    fn v1_header__basic_sample__match() {
        const SAMPLE: &str = "OFXHEADER:100\n\
                              DATA:OFXSGML\n\
                              VERSION:102\n\
                              SECURITY:NONE\n\
                              ENCODING:USASCII\n\
                              CHARSET:1252\n\
                              COMPRESSION:NONE\n\
                              OLDFILEUID:NONE\n\
                              NEWFILEUID:NONE";
        let expected: Vec<HeaderEntry> = vec![
            HeaderEntry::OfxHeader(100u32),
            HeaderEntry::Data(Data::OfxSgml),
            HeaderEntry::Version(102u32),
            HeaderEntry::Security(None),
            HeaderEntry::Encoding(Encoding::UsAscii),
            HeaderEntry::Charset(Some(Charset::Windows1252)),
            HeaderEntry::Compression,
            HeaderEntry::OldFileUid(None),
            HeaderEntry::NewFileUid(None),
        ];

        assert_consumed_eq(v1_header, SAMPLE, expected);
    }

    #[test]
    fn v1_header_entry__valid_charset__match() {
        assert_consumed_eq(
            v1_header_entry,
            "CHARSET:1252",
            HeaderEntry::Charset(Some(Charset::Windows1252)),
        );
    }

    #[test]
    fn v1_header_entry__unknown__match() {
        assert_consumed_eq(
            v1_header_entry,
            "FOO:BAR",
            HeaderEntry::Unknown(String::from("BAR")),
        );
    }

    #[test]
    fn v1_header_entry__no_separator__no_match() {
        assert_not_consumed(v1_header_entry, "CHARSET1252");
    }

    #[test]
    fn header_value__valid_charset__match() {
        assert_consumed_eq(
            |i| header_value("CHARSET", i),
            "1252",
            HeaderEntry::Charset(Some(Charset::Windows1252)),
        );
    }

    #[test]
    fn header_value__unknown__match() {
        assert_consumed_eq(
            |i| header_value("FOO", i),
            "BAR",
            HeaderEntry::Unknown(String::from("BAR")),
        );
    }

    #[test]
    fn header_charset_value__iso88591__match() {
        assert_consumed_eq(header_charset_value, "ISO-8859-1", Some(Charset::Iso88591));
    }

    #[test]
    fn header_charset_value__1252__match() {
        assert_consumed_eq(header_charset_value, "1252", Some(Charset::Windows1252));
    }

    #[test]
    fn header_charset_value__other_input__no_match() {
        assert_not_consumed(header_charset_value, "asdf");
    }

    #[test]
    fn header_data_value__ofxsgml__match() {
        assert_consumed_eq(header_data_value, "OFXSGML", Data::OfxSgml);
    }

    #[test]
    fn header_data_value__other_input__match() {
        assert_consumed_eq(header_data_value, "ASDF", Data::Other(String::from("ASDF")));
    }

    #[test]
    fn header_encoding_value__utf8__match() {
        assert_consumed_eq(header_encoding_value, "UTF-8", Encoding::Utf8);
    }

    #[test]
    fn header_encoding_value__usascii__match() {
        assert_consumed_eq(header_encoding_value, "USASCII", Encoding::UsAscii);
    }

    #[test]
    fn header_encoding_value__other_input__no_match() {
        assert_not_consumed(header_encoding_value, "ASDF");
    }

    #[test]
    fn header_file_id_value__none__match() {
        assert_consumed_eq(header_file_id_value, "NONE", None);
    }

    #[test]
    fn header_file_id_value__other_input__match() {
        assert_consumed_eq(header_file_id_value, "FOO123", Some(String::from("FOO123")));
    }

    #[test]
    fn header_security_value__none__match() {
        assert_consumed_eq(header_security_value, "NONE", None);
    }

    #[test]
    fn header_security_value__type1__match() {
        assert_consumed_eq(header_security_value, "TYPE1", Some(Security::Type1));
    }

    #[test]
    fn header_security_value__other_input__no_match() {
        assert_not_consumed(header_security_value, "ASDF");
    }

    #[test]
    fn none__none_input__match() {
        assert_consumed(none, "NONE");
    }

    #[test]
    fn none__other_input__no_match() {
        assert_not_consumed(none, "NONO");
    }

    #[test]
    fn unsigned_integer__zero__match() {
        assert_consumed_eq(unsigned_integer, "0", 0u32);
    }

    #[test]
    fn unsigned_integer__large_number__match() {
        assert_consumed_eq(unsigned_integer, "1234567890", 1234567890u32);
    }

    #[test]
    fn unsigned_integer__negative__no_match() {
        assert_not_consumed(unsigned_integer, "-123");
    }
}
