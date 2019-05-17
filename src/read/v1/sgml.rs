use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1, take_until, take_while, take_while1},
    character::complete::{char, digit1},
    combinator::map,
    multi::many1,
    sequence::delimited,
    IResult,
};

pub fn sgml_open_tag(i: &str) -> IResult<&str, &str> {
    delimited(char('<'), take_while1(is_uppercase), char('>'))(i)
}

#[allow(clippy::needless_lifetimes)]
pub fn sgml_close_tag<'a>(name: &'a str) -> impl Fn(&'a str) -> IResult<&'a str, ()> {
    move |i: &str| {
        delimited(tag("</"), tag(name), char('>'))(i).map(|(r, _)| (r, ()))
    }
}

pub fn sgml_data1(i: &str) -> IResult<&str, Vec<&str>> {
    const CDATA_END: &str = "]]>";

    let (i, mut results) = many1(
        alt((
            delimited(
                char('&'),
                alt((
                    map(tag("lt"), |_| "<"),
                    map(tag("gt"), |_| ">"),
                    map(tag("amp"), |_| "&"),
                    map(tag("nbsp"), |_| "\u{a0}"),
                )),
                char(';')
            ),
            delimited(
                tag("<!CDATA["),
                take_until(CDATA_END),
                tag(CDATA_END),
            ),
            sgml_data_unescaped,
        )),
    )(i)?;

    let last_idx = results.len() - 1;
    results[last_idx] = results[last_idx].trim_end_matches(is_breaking_whitespace);
    if results[last_idx].is_empty() { let _ = results.pop().unwrap(); }

    Ok((i, results))
}

fn sgml_data_unescaped(i: &str) -> IResult<&str, &str> {
    take_till1(is_sgml_data_end)(i)
}

fn sgml_data_escaped(i: &str) -> IResult<&str, &str> {
    const CDATA_END: &str = "]]>";

    alt((
        delimited(
            char('&'),
            alt((
                map(tag("lt"), |_| "<"),
                map(tag("gt"), |_| ">"),
                map(tag("amp"), |_| "&"),
                map(tag("nbsp"), |_| "\u{a0}"),
            )),
            char(';')
        ),
        delimited(
            tag("<!CDATA["),
            take_until(CDATA_END),
            tag(CDATA_END),
        ),
    ))(i)
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
    digit1(i).map(|(r, v)| (r, v.parse::<u32>().unwrap()))
}

fn is_sgml_data_end(chr: char) -> bool {
    chr == '<' || chr == '&'
}

fn is_not_open_bracket(chr: char) -> bool {
    (chr as u8) != 0x3C
}

fn is_lowercase(chr: char) -> bool {
    let chr = chr as u8;
    chr >= 0x61 && chr <= 0x7A
}

fn is_uppercase(chr: char) -> bool {
    let chr = chr as u8;
    chr >= 0x41 && chr <= 0x5A
}

fn is_whitespace(chr: char) -> bool {
    let chr = chr as u8;
    chr == 0x09 || chr == 0x0A || chr == 0x0D || chr == 0x20
}

fn is_breaking_whitespace(chr: char) -> bool {
    let chr = chr as u8;
    chr == 0x09 || chr == 0x0D || chr == 0x20
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn sgml_open_tag__valid_tag__match() {
        assert_eq!(sgml_open_tag("<FOO>end"), Ok(("end", "FOO")));
    }

    #[test]
    fn sgml_open_tag__lowercase_tag__no_match() {
        assert!(sgml_open_tag("<bad>end").is_err());
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
    fn sgml_data1__data_with_trailing_whitespace__match() {
        assert_eq!(sgml_data1("a sdf 123  </END>"), Ok(("</END>", vec!["a sdf 123"])))
    }

    #[test]
    fn sgml_data1__data_with_escaped_chars__match() {
        assert_eq!(sgml_data1("&lt;test&gt; &amp;asdf&nbsp;</END>"), Ok(("</END>", vec!["<", "test", ">", " ", "&", "asdf", "\u{00a0}"])))
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
        assert_eq!(whitespace(" \t \r\nend"), Ok(("end", " \t \r\n")));
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
    fn is_lowercase__lowercase_a__returns_true() {
        assert!(is_lowercase('a'));
    }

    #[test]
    fn is_lowercase__capital_a__returns_false() {
        assert!(!is_lowercase('A'));
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
