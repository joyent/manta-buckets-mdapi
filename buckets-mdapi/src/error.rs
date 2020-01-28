use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::error;

#[derive(Clone, Copy)]
pub enum BucketsMdapiErrorType {
    BucketAlreadyExists,
    BucketNotFound,
    ObjectNotFound,
    PostgresError,
    LimitConstraintError,
    PreconditionFailedError,
}

impl ToString for BucketsMdapiErrorType {
    fn to_string(&self) -> String {
        match *self {
            BucketsMdapiErrorType::BucketAlreadyExists => "BucketAlreadyExists".into(),
            BucketsMdapiErrorType::BucketNotFound => "BucketNotFound".into(),
            BucketsMdapiErrorType::ObjectNotFound => "ObjectNotFound".into(),
            BucketsMdapiErrorType::PostgresError => "PostgresError".into(),
            BucketsMdapiErrorType::LimitConstraintError => "LimitConstraintError".into(),
            BucketsMdapiErrorType::PreconditionFailedError => "PreconditionFailedError".into(),
        }
    }
}

impl BucketsMdapiErrorType {
    fn message(self) -> String {
        match self {
            BucketsMdapiErrorType::BucketAlreadyExists => "requested bucket already exists".into(),
            BucketsMdapiErrorType::BucketNotFound => "requested bucket not found".into(),
            BucketsMdapiErrorType::ObjectNotFound => "requested object not found".into(),
            BucketsMdapiErrorType::PostgresError => "postgres encountered an error".into(),
            BucketsMdapiErrorType::LimitConstraintError => "a limit constraint was violated".into(),
            BucketsMdapiErrorType::PreconditionFailedError => "precondition failed ? ? ? ".into(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct BucketsMdapiError {
    pub error: BucketsMdapiInnerError,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct BucketsMdapiInnerError {
    pub name: String,
    pub message: String,
}

impl BucketsMdapiError {
    pub fn new(error: BucketsMdapiErrorType) -> Self {
        Self::with_message(error, error.message())
    }

    pub fn with_message(error: BucketsMdapiErrorType, msg: String) -> Self {
        let inner = BucketsMdapiInnerError {
            name: error.to_string(),
            message: msg,
        };
        Self { error: inner }
    }

    pub fn description(&self) -> &str {
        &self.error.message
    }
}

impl fmt::Display for BucketsMdapiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error.message)
    }
}

impl error::Error for BucketsMdapiError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
