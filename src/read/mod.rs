pub mod v1;

use crate::{
    error::OfxError,
    model::Ofx,
};
use std::marker::Sized;

pub trait Read where Self: Sized {
    fn read(input: &str) -> Result<Self, OfxError>;
}

impl Read for Ofx {
    fn read(input: &str) -> Result<Ofx, OfxError> {
        v1::ofx(input)
            .map(|(_, ofx)| ofx)
            .map_err(|_| OfxError::ParseError)
    }
}
