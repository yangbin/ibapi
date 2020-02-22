use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

use crate::ib::order_condition::OrderCondition;
use crate::ib::types::*;
use crate::ib::contract::ShortSaleSlot;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, SmartDefault)]
pub enum OrderType {
    #[default]
    #[serde(rename="")] None,
    MKT,
    LMT,
    STP,
    #[serde(rename="STP LMT")] STP_LMT,
    REL,
    TRAIL,
    #[serde(rename="BOX TOP")]
    BOX_TOP,
    #[serde(rename="FIX PEGGED")] FIX_PEGGED,
    LIT,
    #[serde(rename="LMT + MKT")] LMT_PLUS_MKT,
    LOC,
    MIT,
    #[serde(rename="MKT PRT")] MKT_PRT,
    MOC,
    MTL,
    #[serde(rename="PASSV REL")] PASSV_REL,
    // TODO not supported because we can't dynamically read PegToBenchParams
    // #[serde(rename="PEG BENCH")] PEG_BENCH,
    #[serde(rename="PEG MID")] PEG_MID,
    #[serde(rename="PEG MKT")] PEG_MKT,
    #[serde(rename="PEG PRIM")] PEG_PRIM,
    #[serde(rename="PEG STK")] PEG_STK,
    #[serde(rename="REL + LMT")] REL_PLUS_LMT,
    #[serde(rename="REL + MKT")] REL_PLUS_MKT,
    #[serde(rename="SNAP MID")] SNAP_MID,
    #[serde(rename="SNAP MKT")] SNAP_MKT,
    #[serde(rename="SNAP PRIM")] SNAP_PRIM,
    #[serde(rename="STP PRT")] STP_PRT,
    #[serde(rename="TRAIL LIMIT")] TRAIL_LIMIT,
    #[serde(rename="TRAIL LIT")] TRAIL_LIT,
    #[serde(rename="TRAIL LMT + MKT")] TRAIL_LMT_PLUS_MKT,
    #[serde(rename="TRAIL MIT")] TRAIL_MIT,
    #[serde(rename="TRAIL REL + MKT")] TRAIL_REL_PLUS_MKT,
    VOL,
    VWAP,
    QUOTE,
    #[serde(rename="PPV")] PEG_PRIM_VOL,
    #[serde(rename="PDV")] PEG_MID_VOL,
    #[serde(rename="PMV")] PEG_MKT_VOL,
    #[serde(rename="PSV")] PEG_SRF_VOL,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, SmartDefault)]
pub enum ClearingIntent {
    #[default]
    #[serde(rename="")] Default,
    IB,
    Away,
    PTA,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, SmartDefault)]
pub enum Origin {
    #[default]
    #[serde(rename="0")] Customer,
    #[serde(rename="1")] Firm,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, SmartDefault)]
pub enum AuctionStrategy {
    #[default]
    #[serde(rename="0")] AuctionUnset,
    #[serde(rename="1")] AuctionMatch,
    #[serde(rename="2")] AuctionImprovement,
    #[serde(rename="3")] AuctionTransparent,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, SmartDefault)]
pub struct OrderComboLeg {
    pub price: Option<f64>,
}

/// Institutional orders only
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, SmartDefault)]
pub enum OpenClose {
    #[default]
    #[serde(rename="", alias="?")] None,
    #[serde(rename="O")] Open,
    #[serde(rename="C")] Close,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, SmartDefault)]
pub struct Order {
    // Order IDs
    pub order_id: i32,
    pub client_id: i32,
    pub perm_id: i32,
    /// Parent order Id, to associate Auto STP or TRAIL orders with the original order.
    pub parent_id: i32,
    pub parent_perm_id: Option<i64>,

    // Primary fields
    pub action: Action,
    pub total_quantity: f64,
    pub display_size: i32,
    pub order_type: OrderType,
    pub lmt_price: Option<f64>,
    pub aux_price: Option<f64>,
    pub tif: TimeInForce,

    // Secondary fields
    pub all_or_none: bool,
    pub block_order: bool,
    pub hidden: bool,
    pub outside_rth: bool,
    pub sweep_to_fill: bool,

    /// REL orders only, specify the decimal, e.g. .04 not 4
    pub percent_offset: Option<f64>,

    /// Trailing Stop orders only, specify the percentage, e.g. 3, not .03
    pub trailing_percent: Option<f64>,

    /// Trailing Stop orders only
    pub trail_stop_price: Option<f64>,

    pub min_qty: Option<i32>,

    /// Format: 20060505 08:00:00 EST
    pub good_after_time: String,
    /// Format: 20060505 08:00:00 EST or 20060505
    pub good_till_date: String,

    /// one cancels all group name
    pub oca_group: String,
    pub oca_type: OcaType,
    /// order reference
    pub order_ref: String,

    pub rule80a: Rule80A,
    pub trigger_method: TriggerMethod,

    // Extended fields
    /// for GTC orders
    pub active_start_time: String,
    /// for GTC orders
    pub active_stop_time: String,

    // financial advisors only
    pub fa_group: String,
    pub fa_profile: String,
    pub fa_method: String,
    pub fa_percentage: String,

    // institutional (ie non-cleared) only

    pub open_close: OpenClose,
    pub origin: Origin,
    /// 1 if you hold the shares, 2 if they will be delivered from elsewhere.  Only for Action="SSHORT
    pub short_sale_slot: ShortSaleSlot,
    /// set when slot=2 only.
    pub designated_location: String,
    #[default(-1)]
    pub exempt_code: i32,

    // SMART routing only

    pub discretionary_amt: f64,
    pub e_trade_only: bool,
    pub firm_quote_only: bool,
    pub nbbo_price_cap: Option<f64>,
    pub opt_out_smart_routing: bool,

    // BOX exchange orders only
    pub auction_strategy: Option<i32>,
    pub starting_price: Option<f64>,
    pub stock_ref_price: Option<f64>,
    pub delta: Option<f64>,

    // pegged to stock and VOL orders only
    pub stock_range_lower: Option<f64>,
    pub stock_range_upper: Option<f64>,

    pub randomize_size: bool,
    pub randomize_price: bool,

    // VOLATILITY ORDERS ONLY
    pub volatility: Option<f64>,
    pub volatility_type: VolatilityType,
    pub delta_neutral_order_type: String,
    pub delta_neutral_aux_price: Option<f64>,
    pub delta_neutral_con_id: i32,
    pub delta_neutral_settling_firm: String,
    pub delta_neutral_clearing_account: String,
    pub delta_neutral_clearing_intent: ClearingIntent,
    pub delta_neutral_open_close: OpenClose,
    pub delta_neutral_short_sale: Option<bool>,
    pub delta_neutral_short_sale_slot: ShortSaleSlot,
    pub delta_neutral_designated_location: String,
    pub continuous_update: bool,
    pub reference_price_type: ReferencePriceType,

    // COMBO ORDERS ONLY

    /// EFP orders only
    pub basis_points: Option<f64>,
    /// EFP orders only
    pub basis_points_type: Option<i32>,

    // SCALE ORDERS ONLY
    pub scale_init_level_size: Option<i32>,
    pub scale_subs_level_size: Option<i32>,
    pub scale_price_increment: Option<f64>,
    pub scale_price_adjust_value: Option<f64>,
    pub scale_price_adjust_interval: Option<i32>,
    pub scale_profit_offset: Option<f64>,
    pub scale_auto_reset: bool,
    pub scale_init_position: Option<i32>,
    pub scale_init_fill_qty: Option<i32>,
    pub scale_random_percent: bool,
    pub scale_table: String,

    // HEDGE ORDERS

    /// `hedge_param` is associated data in `HedgeType`
    pub hedge_type: HedgeType,
    /// 'beta=X' value for beta hedge, 'ratio=Y' for pair hedge
    pub hedge_param: String,

    // Clearing info
    /// IB account
    pub account: String,
    pub settling_firm: String,
    /// True beneficiary of the order
    pub clearing_account: String,
    pub clearing_intent: ClearingIntent,

    // ALGO ORDERS ONLY

    pub algo_strategy: String,

    pub algo_params: Vec<TagValue>,
    pub smart_combo_routing_params: Vec<TagValue>,

    pub algo_id: String,

    // processing control
    pub what_if: bool,

    /// if false, order will be created but not transmited
    #[default(true)]
    pub transmit: bool,
    pub override_percentage_constraints: bool,


    pub not_held: bool,
    pub solicited: bool,

    // models

    pub model_code: String,

    // order combo legs
    pub order_combo_legs: Vec<OrderComboLeg>,

    pub order_misc_options: Vec<TagValue>,

    // VER PEG2BENCH fields:

    pub reference_contract_id: i32,
    pub pegged_change_amount: f64,
    pub is_pegged_change_amount_decrease: bool,
    pub reference_change_amount: f64,
    pub reference_exchange_id: String,
    pub adjusted_order_type: String,
    pub trigger_price: Option<f64>,
    pub adjusted_stop_price: Option<f64>,
    pub adjusted_stop_limit_price: Option<f64>,
    pub adjusted_trailing_amount: Option<f64>,
    pub adjustable_trailing_unit: i32,
    pub lmt_price_offset: Option<f64>,

    pub conditions: Vec<OrderCondition>,
    pub conditions_cancel_order: bool,
    pub conditions_ignore_rth: bool,

    // ext operator

    pub ext_operator: String,

    pub soft_dollar_tier: SoftDollarTier,

    // native cash quantity
    pub cash_qty: Option<f64>,

    pub mifid2_decision_maker: String,
    pub mifid2_decision_algo: String,
    pub mifid2_execution_trader: String,
    pub mifid2_execution_algo: String,

    // don't use auto price for hedge

    pub dont_use_auto_price_for_hedge: bool,

    pub is_oms_container: bool,

    pub discretionary_up_to_limit_price: bool,

    pub auto_cancel_date: String,
    pub filled_quantity: Option<f64>,
    pub ref_futures_con_id: i32,
    pub auto_cancel_parent: bool,
    pub shareholder: String,
    pub imbalance_only: bool,
    pub route_marketable_to_bbo: bool,

    pub use_price_mgmt_algo: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, SmartDefault)]
pub struct OrderState {
    pub status: String,

    pub init_margin_before: String,
    pub maint_margin_before: String,
    pub equity_with_loan_before: String,
    pub init_margin_change: String,
    pub maint_margin_change: String,
    pub equity_with_loan_change: String,
    pub init_margin_after: String,
    pub maint_margin_after: String,
    pub equity_with_loan_after: String,

    pub commission: Option<f64>,
    pub min_commission: Option<f64>,
    pub max_commission: Option<f64>,
    pub commission_currency: String,

    pub warning_text: String,

    pub completed_time: String,
    pub completed_status: String,
}
