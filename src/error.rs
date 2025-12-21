use crate::schemas;
use axum::{
    Json,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use validator::ValidationErrors;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{user_message}")]
    BadRequest {
        user_message: String,
        #[source]
        source: Option<anyhow::Error>,
    },

    #[error("{user_message}")]
    NotFound {
        user_message: String,
        #[source]
        source: Option<anyhow::Error>,
    },

    #[error("{user_message}")]
    Conflict {
        user_message: String,
        #[source]
        source: Option<anyhow::Error>,
    },

    #[error("{user_message}")]
    UniqueViolation {
        user_message: String,
        #[source]
        source: Option<anyhow::Error>,
    },

    #[error("{user_message}")]
    Unauthorized {
        user_message: String,
        #[source]
        source: Option<anyhow::Error>,
    },

    #[error("{0}")]
    Validation(#[from] ValidationErrors),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub trait AppErrorResultExt<T> {
    fn into_bad_request(self) -> Result<T, AppError>;
    fn into_not_found(self) -> Result<T, AppError>;
    fn into_conflict(self) -> Result<T, AppError>;
    fn into_unique_violation(self) -> Result<T, AppError>;
    fn into_unauthorized(self) -> Result<T, AppError>;
}

impl<T, E> AppErrorResultExt<T> for Result<T, E>
where
    E: Into<anyhow::Error>,
{
    fn into_bad_request(self) -> Result<T, AppError> {
        self.map_err(AppError::into_bad_request)
    }
    fn into_not_found(self) -> Result<T, AppError> {
        self.map_err(AppError::into_not_found)
    }
    fn into_conflict(self) -> Result<T, AppError> {
        self.map_err(AppError::into_conflict)
    }
    fn into_unique_violation(self) -> Result<T, AppError> {
        self.map_err(AppError::into_unique_violation)
    }
    fn into_unauthorized(self) -> Result<T, AppError> {
        self.map_err(AppError::into_unauthorized)
    }
}

pub trait AppErrorOptionExt<T> {
    fn not_found_msg(self, msg: &str) -> Result<T, AppError>;
    fn bad_request_msg(self, msg: &str) -> Result<T, AppError>;
    fn unauthorized_msg(self, msg: &str) -> Result<T, AppError>;
}

impl<T> AppErrorOptionExt<T> for Option<T> {
    fn not_found_msg(self, msg: &str) -> Result<T, AppError> {
        self.ok_or_else(|| AppError::not_found(msg))
    }
    fn bad_request_msg(self, msg: &str) -> Result<T, AppError> {
        self.ok_or_else(|| AppError::bad_request(msg))
    }
    fn unauthorized_msg(self, msg: &str) -> Result<T, AppError> {
        self.ok_or_else(|| AppError::unauthorized(msg))
    }
}

impl AppError {
    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::BadRequest {
            user_message: msg.into(),
            source: None,
        }
    }
    pub fn into_bad_request(err: impl Into<anyhow::Error>) -> Self {
        let e = err.into();
        Self::BadRequest {
            user_message: e.to_string(),
            source: Some(e),
        }
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound {
            user_message: msg.into(),
            source: None,
        }
    }
    pub fn into_not_found(err: impl Into<anyhow::Error>) -> Self {
        let e = err.into();
        Self::NotFound {
            user_message: e.to_string(),
            source: Some(e),
        }
    }

    pub fn conflict(msg: impl Into<String>) -> Self {
        Self::Conflict {
            user_message: msg.into(),
            source: None,
        }
    }
    pub fn into_conflict(err: impl Into<anyhow::Error>) -> Self {
        let e = err.into();
        Self::Conflict {
            user_message: e.to_string(),
            source: Some(e),
        }
    }

    pub fn unique_violation(msg: impl Into<String>) -> Self {
        Self::UniqueViolation {
            user_message: msg.into(),
            source: None,
        }
    }
    pub fn into_unique_violation(err: impl Into<anyhow::Error>) -> Self {
        let e = err.into();
        Self::UniqueViolation {
            user_message: e.to_string(),
            source: Some(e),
        }
    }

    pub fn unauthorized(msg: impl Into<String>) -> Self {
        Self::Unauthorized {
            user_message: msg.into(),
            source: None,
        }
    }
    pub fn into_unauthorized(err: impl Into<anyhow::Error>) -> Self {
        let e = err.into();
        Self::Unauthorized {
            user_message: e.to_string(),
            source: Some(e),
        }
    }

    pub fn from_generic_error(err: impl Into<anyhow::Error>) -> Self {
        Self::Other(err.into())
    }

    fn source_error(&self) -> Option<&anyhow::Error> {
        match self {
            AppError::BadRequest { source, .. }
            | AppError::NotFound { source, .. }
            | AppError::Conflict { source, .. }
            | AppError::UniqueViolation { source, .. }
            | AppError::Unauthorized { source, .. } => source.as_ref(),
            _ => None,
        }
    }

    fn get_tag(&self) -> &'static str {
        match self {
            AppError::BadRequest { .. } => "bad_request",
            AppError::NotFound { .. } => "not_found",
            AppError::Conflict { .. } => "conflict",
            AppError::UniqueViolation { .. } => "unique_violation",
            AppError::Unauthorized { .. } => "unauthorized",
            AppError::Validation(..) => "validation",
            AppError::Other(..) => "other",
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let tag = self.get_tag();

        let (status_code, client_message) = match &self {
            AppError::BadRequest { user_message, .. } => {
                (StatusCode::BAD_REQUEST, user_message.as_str())
            }
            AppError::NotFound { user_message, .. } => {
                (StatusCode::NOT_FOUND, user_message.as_str())
            }
            AppError::Conflict { user_message, .. }
            | AppError::UniqueViolation { user_message, .. } => {
                (StatusCode::CONFLICT, user_message.as_str())
            }
            AppError::Unauthorized { user_message, .. } => {
                (StatusCode::UNAUTHORIZED, user_message.as_str())
            }
            AppError::Validation(validation_errors) => {
                let msg = validation_errors
                    .field_errors()
                    .values()
                    .flat_map(|v| v.iter())
                    .find_map(|e| e.message.as_ref().map(|m| m.as_ref()))
                    .unwrap_or("invalid input");

                (StatusCode::BAD_REQUEST, msg)
            }
            AppError::Other(_) => (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong"),
        };

        match &self {
            AppError::Other(error) => {
                log::error!("[{}] unexpected error: {:?}", tag, error);
            }
            _ => {
                if let Some(source) = self.source_error() {
                    log::info!("[{}] {}: {:?}", tag, client_message, source);
                } else {
                    log::info!("[{}] {}", tag, client_message);
                }
            }
        }

        (
            status_code,
            [(header::CONTENT_TYPE, "application/json")],
            Json(schemas::error::Response {
                status: tag.to_string(),
                message: client_message.to_string(),
            }),
        )
            .into_response()
    }
}

macro_rules! impl_from_error {
    ($($t:ty),+ $(,)?) => {
        $(
            impl From<$t> for AppError {
                fn from(err: $t) -> Self {
                    Self::Other(anyhow::Error::new(err))
                }
            }
        )+
    };
}

impl_from_error!(std::io::Error, std::string::FromUtf8Error);
