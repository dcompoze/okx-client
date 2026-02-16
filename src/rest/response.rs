use serde::Deserialize;

use crate::error::OkxError;

/// Raw OKX API response wrapper.
///
/// All OKX REST responses follow this structure:
/// ```json
/// { "code": "0", "msg": "", "data": [...] }
/// ```
/// On success, `code` is `"0"` and `msg` is empty.
/// On error, `code` is a non-zero error code and `msg` contains the error message.
#[derive(Debug, Deserialize)]
pub(crate) struct OkxResponse<T> {
    pub code: String,
    pub msg: String,
    pub data: T,
}

impl<T> OkxResponse<T> {
    /// Unwrap the response, returning `data` on success or an error on failure.
    pub fn into_result(self) -> Result<T, OkxError> {
        if self.code == "0" {
            Ok(self.data)
        } else {
            Err(OkxError::Api {
                code: self.code,
                msg: self.msg,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_response() {
        let json = r#"{"code":"0","msg":"","data":[{"balance":"100.5"}]}"#;
        let resp: OkxResponse<Vec<serde_json::Value>> = serde_json::from_str(json).unwrap();
        let data = resp.into_result().unwrap();
        assert_eq!(data.len(), 1);
    }

    #[test]
    fn test_error_response() {
        let json = r#"{"code":"51008","msg":"Order failed. Insufficient balance.","data":[]}"#;
        let resp: OkxResponse<Vec<serde_json::Value>> = serde_json::from_str(json).unwrap();
        let err = resp.into_result().unwrap_err();
        match err {
            OkxError::Api { code, msg } => {
                assert_eq!(code, "51008");
                assert!(msg.contains("Insufficient"));
            }
            _ => panic!("Expected Api error"),
        }
    }
}
