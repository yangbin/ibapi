use serde::{Serialize, Deserialize};

use crate::ib::*;
use crate::ib::types::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Hello {
    server_version: u64,
    server_connection_time: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    #[serde(rename="4")]
    ErrMsg { version: i32, id: i32, error_code: i32, error_msg: String },
    #[serde(rename="5")]
    OpenOrder, // TODO
    #[serde(rename="6")]
    AcctValue { version: i32, key: String, val: String, cur: String, account_name: String },
    #[serde(rename="7")]
    PortfolioValue { version: i32, contract: PortfolioValueContract, position: f64, market_price: f64, market_value: f64, average_cost: f64, unrealized_pnl: f64, realized_pnl: f64, account_name: String },
    #[serde(rename="8")]
    AcctTime { version: i32, account_time: String },
    #[serde(rename="9")]
    NextValidId { version: i32, order_id: i32 },
    #[serde(rename="15")]
    ManagedAccts { version: i32, accounts_list: String },
    #[serde(rename="54")]
    AcctDownloadEnd { version: i32, account: String },
    #[serde(rename="61")]
    PositionData { version: i32, account: String, contract: PositionDataContract, position: f64, avg_cost: f64 },
    #[serde(rename="62")]
    PositionDataEnd { version: i32 },

    /// Not actual IB message, used to encode an unknown message
    UnknownMessage(String),
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PositionDataContract {
    pub conid: i32,
    pub symbol: String,
    pub sec_type: String,
    pub last_trade_date_or_contract_month: String,
    pub strike: f64,
    pub right: Right,
    pub multiplier: String, // should be double
    pub exchange: String,
    pub currency: String,
    pub local_symbol: String,
    pub trading_class: String,
}

impl From<PositionDataContract> for Contract {
    fn from(c: PositionDataContract) -> Contract {
        Contract {
            conid: c.conid,
            symbol: c.symbol,
            sec_type: c.sec_type,
            last_trade_date_or_contract_month: c.last_trade_date_or_contract_month,
            strike: c.strike,
            right: c.right,
            multiplier: c.multiplier,
            exchange: c.exchange,
            currency: c.currency,
            local_symbol: c.local_symbol,
            trading_class: c.trading_class,
            .. Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PortfolioValueContract {
    pub conid: i32,
    pub symbol: String,
    pub sec_type: String,
    pub last_trade_date_or_contract_month: String,
    pub strike: f64,
    pub right: Right,
    pub multiplier: String, // should be double
    pub primary_exch: String,
    pub currency: String,
    pub local_symbol: String,
    pub trading_class: String,
}

impl From<PortfolioValueContract> for Contract {
    fn from(c: PortfolioValueContract) -> Contract {
        Contract {
            conid: c.conid,
            symbol: c.symbol,
            sec_type: c.sec_type,
            last_trade_date_or_contract_month: c.last_trade_date_or_contract_month,
            strike: c.strike,
            right: c.right,
            multiplier: c.multiplier,
            primary_exch: c.primary_exch,
            currency: c.currency,
            local_symbol: c.local_symbol,
            trading_class: c.trading_class,
            .. Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct  DeltaNeutralContract {
    pub conid: i32,
    pub delta: f64,
    pub price: f64,
}
