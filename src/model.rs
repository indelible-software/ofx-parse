use chrono::{DateTime, FixedOffset};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ofx {
    pub headers: Vec<HeaderEntry>,
}

/// OFX Section 2.4.5
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MessageSetAggregate {
    SignonRequestsV1,
    SignonRequestsV2,
    SignonResponsesV1,
    SignonResponsesV2,
    SignupRequestsV1,
    SignupRequestsV2,
    SignupResponsesV1,
    SignupResponsesV2,
    BankingRequestsV1,
    BankingRequestsV2,
    BankingResponsesV1,
    BankingResponsesV2,
    CreditCardStatementsRequestsV1,
    CreditCardStatementsRequestsV2,
    CreditCardStatementsResponsesV1,
    CreditCardStatementsResponsesV2,
    LoanStatementsRequestsV1,
    LoanStatementsResponsesV1,
    InvestmentStatementsRequestsV1,
    InvestmentStatementsRequestsV2,
    InvestmentStatementsResponsesV1,
    InvestmentStatementsResponsesV2,
    InterbankFundsTransfersRequestsV1,
    InterbankFundsTransfersRequestsV2,
    InterbankFundsTransfersResponsesV1,
    InterbankFundsTransfersResponsesV2,
    WireFundsTransfersRequestsV1,
    WireFundsTransfersRequestsV2,
    WireFundsTransfersResponsesV1,
    WireFundsTransfersResponsesV2,
    PaymentsRequestsV1,
    PaymentsRequestsV2,
    PaymentsResponsesV1,
    PaymentsResponsesV2,
    GeneralEmailRequestsV1,
    GeneralEmailRequestsV2,
    GeneralEmailResponsesV1,
    GeneralEmailResponsesV2,
    InvestmentSecurityListRequestsV1,
    InvestmentSecurityListRequestsV2,
    InvestmentSecurityListResponsesV1,
    InvestmentSecurityListResponsesV2,
    BillerDirectoryRequestsV1,
    BillerDirectoryResponsesV1,
    BillDeliveryRequestsV1,
    BillDeliveryResponsesV1,
    FIProfileRequestsV1,
    FIProfileRequestsV2,
    FIProfileResponsesV1,
    FIProfileResponsesV2,
    ImageDownloadRequestsV1,
    ImageDownloadResponsesV1,
    Other(String),
}

/// OFX Section 2.5.1.6
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignonResponseMessage {
    status: Status,
    dt_server: DateTime<FixedOffset>,
    // TODO
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Status {
    // TODO: Use Appendix A to define a StatusCode type
    code: u32,
    severity: Severity,
    message: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Severity {
    Error,
    Info,
    Warn,
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
