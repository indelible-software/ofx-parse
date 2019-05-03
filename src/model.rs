pub enum HeaderEntry {
    Charset(Option<Charset>),
    Compression,
    Data(String),
    Encoding(Encoding),
    NewFileUid(String),
    OfxHeader(u32),
    OldFileUid(String),
    Security(Option<Security>),
    Version(u32),
    Unknown(String),
}

pub enum HeaderElement {
    Charset,
    Compression,
    Data,
    Encoding,
    NewFileUid,
    OfxHeader,
    OldFileUid,
    Security,
    Version,
    Unknown(String),
}

pub enum Charset {
    Iso88591,
    Windows1252,
}

pub enum Encoding {
    UsAscii,
    Utf8,
}

pub enum Security {
    Type1,
}
