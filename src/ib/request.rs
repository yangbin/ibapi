use serde::{Deserialize, Serialize, Serializer};
use smart_default::SmartDefault;

use crate::ib::{Contract, TagValue};

/// Outgoing messages. We use serde rename to the right ID + Version.
/// Also see EClient.h / EClient.cpp
#[derive(Debug, Deserialize, Serialize, SmartDefault)]
pub enum Request {
    #[default] None,
    #[serde(rename="1\011", serialize_with="req_mkt_data")]
    ReqMktData { ticker_id: i32, contract: Contract, generic_tick_list: String, snapshot: bool, regulatory_snapshot: bool, mkt_data_options: Vec<TagValue> },
    #[serde(rename="5\01")]
    ReqOpenOrders,
    #[serde(rename="6\02")]
    ReqAcctData { subscribe: bool, acct_code: String },
    #[serde(rename="16\01")]
    ReqAllOpenOrders,
    #[serde(rename="61\01")]
    ReqPositions,
    #[serde(rename="71\02")]
    StartApi { client_id: i32, optional_capabilities: String },
}

fn req_mkt_data<S: Serializer>(ticker_id: &i32, contract: &Contract, generic_tick_list: &String, snapshot: &bool, regulatory_snapshot: &bool, mkt_data_options: &Vec<TagValue>, s: S) -> Result<S::Ok, S::Error> {
    let has_delta_neutral_contract = contract.delta_neutral_contract.is_some();

    (
        ticker_id,
        contract,
        &contract.combo_legs,
        has_delta_neutral_contract,
        //&contract.delta_neutral_contract,
        generic_tick_list,
        snapshot,
        regulatory_snapshot,
        mkt_data_options
    ).serialize(s)
}

//1-11-1-383430121-ESTC-OPT-20200717-60-P-100-SMART--USD-ESTC  200717P00060000-ESTC-0-0--221-0-
