use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

use crate::ib::types::TriggerMethod;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, SmartDefault)]
pub enum AndOr {
    #[default]
    #[serde(rename="a")] And,
    #[serde(rename="o")] Or,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, SmartDefault)]
pub enum OrderCondition {
    #[default]
    #[serde(rename="1")] Price { and_or: AndOr, trigger_method: TriggerMethod },
    #[serde(rename="3")] Time { and_or: AndOr },
    #[serde(rename="4")] Margin { and_or: AndOr },
    #[serde(rename="5")] Execution { and_or: AndOr, sec_type: String, exchange: String, symbol: String },
    #[serde(rename="6")] Volume { and_or: AndOr },
    #[serde(rename="7")] PercentChange { and_or: AndOr },
}
