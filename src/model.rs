#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ofx {
    pub headers: Vec<HeaderEntry>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HeaderEntry {
    Charset(Option<Charset>),
    Compression,
    Data(Data),
    Encoding(Encoding),
    NewFileUid(Option<String>),
    OfxHeader(u32),
    OldFileUid(Option<String>),
    Security(Option<Security>),
    Version(u32),
    Unknown(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Charset {
    Iso88591,
    Windows1252,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Data {
    OfxSgml,
    Other(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Encoding {
    UsAscii,
    Utf8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Security {
    Type1,
}
