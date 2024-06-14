use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ResponseStatus {
    Success,
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthCheckResponse {
    pub status: ResponseStatus,
    pub message: String
}

