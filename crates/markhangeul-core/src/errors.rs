use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParseError {
    pub code: String,
    pub message: String,
    pub index: usize,
    pub length: usize,
    pub severity: Severity,
}

impl ParseError {
    pub fn error(
        code: impl Into<String>,
        message: impl Into<String>,
        index: usize,
        length: usize,
    ) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            index,
            length: length.max(1),
            severity: Severity::Error,
        }
    }
}
