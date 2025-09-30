use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

// default user &'static
pub const MESSAGE_SUCCESS: &str = "success";
pub const MESSAGE_FAILURE: &str = "failure";
pub const MESSAGE_PARAMETER_ERROR: &str = "parameter error";
pub const MESSAGE_ACCOUNT_ALREADY_EXISTS: &str = "account already exists";
pub const MESSAGE_ACCOUNT_NOT_EXISTS: &str = "account not exists";
pub const MESSAGE_WRONG_ACCOUNT_OR_PASSWORD: &str = "wrong account or password";
pub const MESSAGE_DATE_OPERATION_ERROR: &str = "database operation error";
pub const MESSAGE_INTERNAL_SERVER_ERROR: &str = "internal server error";

pub const CODE_SUCCESS: u16 = 0;
pub const CODE_FAILURE: u16 = 9999;
pub const CODE_PARAMETER_ERROR: u16 = 10000;
pub const CODE_ACCOUNT_ALREADY_EXISTS: u16 = 10001;
pub const CODE_ACCOUNT_NOT_EXISTS: u16 = 10002;
pub const CODE_WRONG_ACCOUNT_OR_PASSWORD: u16 = 10003;
pub const CODE_DATE_OPERATION_ERROR: u16 = 10004;
pub const CODE_INTERNAL_SERVER_ERROR: u16 = 10005;

static GLOBAL_DATA_REPLY: Lazy<Mutex<HashMap<u16, &'static str>>> = Lazy::new(|| {
    let mut m: HashMap<u16, &str> = HashMap::new();
    m.insert(CODE_SUCCESS, MESSAGE_SUCCESS);
    m.insert(CODE_FAILURE, MESSAGE_FAILURE);
    m.insert(CODE_PARAMETER_ERROR, MESSAGE_PARAMETER_ERROR);
    m.insert(CODE_ACCOUNT_ALREADY_EXISTS, MESSAGE_ACCOUNT_ALREADY_EXISTS);
    m.insert(CODE_ACCOUNT_NOT_EXISTS, MESSAGE_ACCOUNT_NOT_EXISTS);
    m.insert(
        CODE_WRONG_ACCOUNT_OR_PASSWORD,
        MESSAGE_WRONG_ACCOUNT_OR_PASSWORD,
    );
    m.insert(CODE_DATE_OPERATION_ERROR, MESSAGE_DATE_OPERATION_ERROR);
    m.insert(CODE_INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR);
    Mutex::new(m)
});

pub fn get_string_value(n: u16) -> &'static str {
    GLOBAL_DATA_REPLY
        .lock()
        .unwrap()
        .get(&n)
        .unwrap_or(&"unknown")
}
