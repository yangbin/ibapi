use serde::{Deserialize, Serialize};

/// Outgoing messages. We use serde rename to the right ID + Version.
/// Also see EClient.h / EClient.cpp
#[derive(Debug, Deserialize, Serialize)]
pub enum Request {
    #[serde(rename="6\02")]
    ReqAcctData { subscribe: bool, acct_code: String },
    #[serde(rename="16\01")]
    ReqAllOpenOrders,
    #[serde(rename="61\01")]
    ReqPositions,
    #[serde(rename="71\02")]
    StartApi { client_id: i32, optional_capabilities: String },
}
