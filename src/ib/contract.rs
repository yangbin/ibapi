use serde::{Deserialize, Serialize};

use crate::ib::types::*;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Contract {
    pub conid: i32,
    pub symbol: String,
    pub sec_type: String,
    pub last_trade_date_or_contract_month: String,
    pub strike: f64,
    pub right: Right,
    pub multiplier: String, // should be double
    pub exchange: String,
    pub primary_exch: String, // pick a non-aggregate (ie not the SMART exchange) exchange that the contract trades on.  DO NOT SET TO SMART.
    pub currency: String,
    pub local_symbol: String,
    pub trading_class: String,
    pub sec_id_type: String, // CUSIP;SEDOL;ISIN;RIC
    pub sec_id: String,

    pub delta_neutral_contract: Option<DeltaNeutralContract>,
    pub include_expired: bool,  // can not be set to true for orders

    // COMBOS
    pub combo_legs_descrip: String, // received in open order version 14 and up for all combos 
    pub combo_legs: Vec<ComboLeg>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DeltaNeutralContract {
    pub conid: i32,
    pub delta: f64,
    pub price: f64,
}

impl Contract {
    pub fn is_combo(&self) -> bool {
        self.combo_legs.len() > 0
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum LegOpenClose {
    /// open/close leg value is same as combo
    #[serde(rename="0")] Same,
    #[serde(rename="1")] Open,
    #[serde(rename="2")] Close,
    #[serde(rename="3")] Unknown,
}

impl Default for LegOpenClose { fn default() -> LegOpenClose { LegOpenClose::Same } }

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ShortSaleSlot {
    #[serde(rename="0")] None,
    #[serde(rename="1")] ClearingBroker,
    #[serde(rename="2")] ThirdParty,
}

impl Default for ShortSaleSlot { fn default() -> ShortSaleSlot { ShortSaleSlot::None } }

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ComboLeg {
    pub con_id: i32,
    pub ratio: i32,
    pub action: Action,

    pub exchange: String,
    pub open_close: LegOpenClose, 

    /// for stock legs when doing short sale
    pub short_sale_slot: ShortSaleSlot,
    pub designated_location: String,
    pub exempt_code: i32,
}

impl Default for ComboLeg {
    fn default() -> ComboLeg {
        ComboLeg {
            con_id: 0,
            ratio: 0,
            action: Default::default(),
            exchange: Default::default(),
            open_close: Default::default(),
            short_sale_slot: Default::default(),
            designated_location: Default::default(),
            exempt_code: -1,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ContractDetails {
    pub contract:             Contract,
    pub market_name:          String,
    pub min_tick:             f64,
    pub order_types:          String,
    pub valid_exchanges:      String,
    pub price_magnifier:      i32,
    pub under_con_id:         i32,
    pub long_name:            String,
    pub contract_month:       String,
    pub industry:             String,
    pub category:             String,
    pub subcategory:          String,
    pub time_zone_id:         String,
    pub trading_hours:        String,
    pub liquid_hours:         String,
    pub ev_rule:              String,
    pub ev_multiplier:        f64,
    pub md_size_multiplier:   i32,
    pub agg_group:            i32,
    pub under_symbol:         String,
    pub under_sec_type:       String,
    pub market_rule_ids:      String,
    pub real_expiration_date: String,
    pub last_trade_time:      String,

    pub sec_id_list:          Vec<TagValue>,

    // BOND value,
    pub cusip:               String,
    pub ratings:             String,
    pub desc_append:         String,
    pub bond_type:           String,
    pub coupon_type:         String,
    pub callable:            bool,
    pub putable:             bool,
    pub coupon:              f64,
    pub convertible:         bool,
    pub maturity:            String,
    pub issue_date:          String,
    pub next_option_date:    String,
    pub next_option_type:    String,
    pub next_option_partial: bool,
    pub notes:               String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ContractDescription {
    pub contract: Contract,
    pub derivative_sec_types: Vec<String>,
}
