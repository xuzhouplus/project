use serde::Serialize;

pub const CODE_OK: i32 = 0;
pub const CODE_NOT_FOUND: i32 = 404;
pub const CODE_INTERNAL: i32 = 500;

#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

#[derive(Debug, Clone, Serialize)]
pub struct IdResult {
    pub id: i64,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: CODE_OK,
            message: "success".into(),
            data: Some(data),
        }
    }

    pub fn error(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }

    pub fn not_found(resource: &str, id: i64) -> Self {
        Self::error(CODE_NOT_FOUND, format!("{resource} {id} not found"))
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::error(CODE_INTERNAL, message)
    }

    pub fn from_result(result: Result<T, impl ToString>) -> Self {
        match result {
            Ok(data) => Self::success(data),
            Err(error) => Self::internal(error.to_string()),
        }
    }
}
