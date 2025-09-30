use crate::constants::constants;
use serde::Serialize;

#[derive(Serialize)]
pub struct Reply<T> {
    pub code: u16,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> Reply<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: constants::CODE_SUCCESS,
            msg: constants::get_string_value(constants::CODE_SUCCESS).to_string(),
            data: Some(data),
        }
    }

    /*
    pub fn error(code: u16, msg: impl Into<String>) -> Self {
        Self {
            code,
            msg: msg.into(),
            data: None,
        }
    }
    */

    pub fn error(code: u16) -> Self {
        Self {
            code,
            msg: constants::get_string_value(code).to_string(),
            data: None,
        }
    }
}
