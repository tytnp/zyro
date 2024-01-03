use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiRet<T> {
    pub ok: bool,
    pub data: Option<T>,
    pub err: Option<String>,
}

impl<T> ApiRet<T> {
    pub fn new() -> Self {
        ApiRet {
            ok: false,
            data: None,
            err: None,
        }
    }

    pub fn with_data(data: T) -> Self {
        ApiRet {
            ok: true,
            data: Some(data),
            err: None,
        }
    }

    pub fn with_error(err: anyhow::Error) -> Self {
        ApiRet {
            ok: false,
            data: None,
            err: Some(err.to_string()),
        }
    }
}