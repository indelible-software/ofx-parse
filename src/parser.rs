use nom::{
    IResult,
    branch::alt,
    bytes::streaming::tag,
    character::streaming::{char, line_ending as eol},
    combinator::map,
    sequence::separated_pair,
};

use crate::model::{Charset, HeaderEntry};

fn end_of_line(i: &str) -> IResult<&str, &str> {
    if i.is_empty() { Ok((i, i)) } else { eol(i) }
}

fn none(i: &str) -> IResult<&str, &str> {
    tag("NONE")(i)
}

fn ofx(i: &str) -> IResult<&str, &str> {
    tag("test")(i)
}

fn v1_header(i: &str) -> IResult<&str, &str> {
    unimplemented!()
}

fn v1_header_entry(i: &str) -> IResult<&str, HeaderEntry> {
    alt((
        map(separated_pair(tag("CHARSET"), char(':'), charset_value), |(_, v)| HeaderEntry::Charset(v)),
        map(separated_pair(tag("CHARSET"), char(':'), charset_value), |(_, v)| HeaderEntry::Charset(v)),
    ))(i)
}

fn charset_value(i: &str) -> IResult<&str, Option<Charset>> {
    alt((
        map(none, |_| None),
        map(tag("ISO-8859-1"), |_| Some(Charset::Iso88591)),
        map(tag("1252"), |_| Some(Charset::Windows1252)),
    ))(i)

/*
    let element = match i {
        "CHARSET" -> HeaderElement::Charset,
        "COMPRESSION" -> HeaderElement::Compression,
        "DATA" -> HeaderElement::Data,
        "ENCODING" -> HeaderElement::Encoding,
        "NEWFILEUID" -> HeaderElement::NewFileUid,
        "OFXHEADER" -> HeaderElement::OfxHeader,
        "OLDFILEUID" -> HeaderElement::OldFileUid,
        "SECURITY" -> HeaderElement::Security,
        "VERSION" -> HeaderElement::Version,
        _ -> HeaderElement::Unknown(String::from(i)),
    }
    */
}

fn header_element_charset(i: &str) -> IResult<&str, &str> {
    tag("CHARSET")(i)
}

fn header_value(i: &str) -> IResult<&str, &str> {
    unimplemented!()
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn end_of_line__lf__match() {
        assert_eq!(end_of_line("\nasdf"), Ok(("asdf", "\n")));
    }

    #[test]
    fn end_of_line__crlf__match() {
        assert_eq!(end_of_line("\r\nasdf"), Ok(("asdf", "\r\n")));
    }

    #[test]
    fn end_of_line__mid_line__no_match() {
        assert!(end_of_line("as\ndf").is_err());
    }

    #[test]
    fn ofx_test() {
        assert_eq!(ofx("test"), Ok(("", "test")));
    }
}
