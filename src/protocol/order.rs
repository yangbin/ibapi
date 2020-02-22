use serde::{Deserialize, Deserializer};

use crate::ib::{Contract, Order, OrderCondition, OrderState};
use crate::ib::contract::{ComboLeg, DeltaNeutralContract, ShortSaleSlot};
use crate::ib::message::{OpenOrder};
use crate::ib::order::{ClearingIntent, OpenClose, OrderComboLeg, OrderType, Origin};
use crate::ib::types::*;

#[derive(Deserialize)]
pub struct OpenOrderMessage {
    order_id: i32,
    contract: ContractFields,
    order: OrderBasic,
    client_id: i32,
    perm_id: i32,
    outside_rth: bool,
    hidden: bool,
    discretionary_amt: f64,
    good_after_time: String,
    _shares_allocation: String, // deprecated
    fa_params: FAParams,
    model_code: String,
    good_till_date: String,
    rule80a: Rule80A,
    percent_offset: Option<f64>,
    settling_firm: String,
    short_sale_params: ShortSaleParams,
    auction_strategy: Option<i32>,
    box_order_params: BoxOrderParams,
    peg_to_stk_or_vol_order_params: PegToStkOrVolOrderParams,
    display_size: Option<i32>,
    block_order: bool,
    sweep_to_fill: bool,
    all_or_none: bool,
    min_qty: Option<i32>,
    oca_type: OcaType,
    e_trade_only: bool,
    firm_quote_only: bool,
    nbbo_price_cap: Option<f64>,
    parent_id: i32,
    trigger_method: TriggerMethod,
    vol_order_params: VolOrderParams<OpenOrderAttribs>,
    trail_params: TrailParams,
    basis_points: BasisPoints,
    combo_legs: ComboLegs,
    smart_combo_routing_params: Vec<TagValue>,
    scale_order_params: ScaleOrderParams,
    hedge_type: HedgeType,
    opt_out_smart_routing: bool,
    clearing_params: ClearingParams,
    not_held: bool,
    c_delta_neutral: DeltaNeutral,
    algo: Option<(String, Vec<TagValue>)>,
    solicited: bool,
    what_if_info_and_commission: WhatIfInfoAndCommission,
    vol_randomize_flags: VolRandomizeFlags,
    // TODO only if PEG_BENCH: peg_to_bench_params: PegToBenchParams,
    conditions: Conditions,
    adjusted_order_params: AdjustedOrderParams,
    soft_dollar_tier: SoftDollarTier,
    cash_qty: Option<f64>,
    dont_use_auto_price_for_hedge: bool,
    is_oms_container: bool,
    discretionary_up_to_limit_price: bool,
    use_price_mgmt_algo: Option<bool>,
}

impl From<OpenOrderMessage> for OpenOrder {
    fn from(f: OpenOrderMessage) -> OpenOrder {
        let mut order = Order::default();
        let mut contract = Contract::default();
        let mut state = OrderState::default();

        order.order_id = f.order_id;
        contract.conid = f.contract.conid;
        contract.symbol = f.contract.symbol;
        contract.sec_type = f.contract.sec_type;
        contract.last_trade_date_or_contract_month = f.contract.last_trade_date_or_contract_month;
        contract.strike = f.contract.strike;
        contract.right = f.contract.right;
        contract.multiplier = f.contract.multiplier;
        contract.exchange = f.contract.exchange;
        contract.currency = f.contract.currency;
        contract.local_symbol = f.contract.local_symbol;
        contract.trading_class = f.contract.trading_class;
        order.action = f.order.action;
        order.total_quantity = f.order.total_quantity;
        order.order_type = f.order.order_type;
        order.lmt_price = f.order.lmt_price;
        order.aux_price = f.order.aux_price;
        order.tif = f.order.tif;
        order.oca_group = f.order.oca_group;
        order.account = f.order.account;
        order.open_close = f.order.open_close;
        order.origin = f.order.origin;
        order.order_ref = f.order.order_ref;
        order.client_id = f.client_id;
        order.perm_id = f.perm_id;
        order.outside_rth = f.outside_rth;
        order.hidden = f.hidden;
        order.discretionary_amt = f.discretionary_amt;
        order.good_after_time = f.good_after_time;
        f._shares_allocation; // deprecated
        order.fa_group = f.fa_params.group;
        order.fa_method = f.fa_params.method;
        order.fa_percentage = f.fa_params.percentage;
        order.fa_profile = f.fa_params.profile;
        order.model_code = f.model_code;
        order.good_till_date = f.good_till_date;
        order.rule80a = f.rule80a;
        order.percent_offset = f.percent_offset;
        order.settling_firm = f.settling_firm;
        order.short_sale_slot = f.short_sale_params.short_sale_slot;
        order.designated_location = f.short_sale_params.designated_location;
        order.exempt_code = f.short_sale_params.exempt_code;
        order.auction_strategy = f.auction_strategy;
        order.starting_price = f.box_order_params.starting_price;
        order.stock_ref_price = f.box_order_params.stock_ref_price;
        order.delta = f.box_order_params.delta;
        order.stock_range_lower = f.peg_to_stk_or_vol_order_params.stock_range_lower;
        order.stock_range_upper = f.peg_to_stk_or_vol_order_params.stock_range_upper;
        order.display_size = f.display_size.unwrap_or_default();
        order.block_order = f.block_order;
        order.sweep_to_fill = f.sweep_to_fill;
        order.all_or_none = f.all_or_none;
        order.min_qty = f.min_qty;
        order.oca_type = f.oca_type;
        order.e_trade_only = f.e_trade_only;
        order.firm_quote_only = f.firm_quote_only;
        order.nbbo_price_cap = f.nbbo_price_cap;
        order.parent_id = f.parent_id;
        order.trigger_method = f.trigger_method;
        let params = f.vol_order_params;
        order.volatility = params.volatility;
        order.volatility_type = params.volatility_type;
        order.delta_neutral_order_type = "None".into();
        order.delta_neutral_aux_price = params.delta_neutral_aux_price;
        let dn = params.delta_neutral_params;
        order.delta_neutral_con_id = dn.delta_neutral_con_id.unwrap_or_default();
        order.delta_neutral_settling_firm = dn.open_order_attribs.settling_firm;
        order.delta_neutral_clearing_account = dn.open_order_attribs.clearing_account;
        order.delta_neutral_clearing_intent = dn.open_order_attribs.clearing_intent;
        order.delta_neutral_open_close = dn.open_order_attribs.open_close;
        order.delta_neutral_short_sale = dn.delta_neutral_short_sale;
        order.delta_neutral_short_sale_slot = dn.delta_neutral_short_sale_slot;
        order.delta_neutral_designated_location = dn.delta_neutral_designated_location;
        order.continuous_update = params.continuous_update;
        order.reference_price_type = params.reference_price_type;
        order.trail_stop_price = f.trail_params.trail_stop_price;
        order.trailing_percent = f.trail_params.trailing_percent;
        order.basis_points = f.basis_points.basis_points;
        order.basis_points_type = f.basis_points.basis_points_type;
        let combo_legs = f.combo_legs;
        contract.combo_legs_descrip = combo_legs.c_combo_legs_descrip;
        contract.combo_legs = combo_legs.c_combo_legs;
        order.order_combo_legs = combo_legs.order_combo_legs;
        order.smart_combo_routing_params = f.smart_combo_routing_params;
        let params = f.scale_order_params;
        order.scale_init_level_size = params.scale_init_level_size;
        order.scale_subs_level_size = params.scale_subs_level_size;
        // TODO order.scale_price_increment = params.scale_price_increment;
        order.hedge_type = f.hedge_type;
        order.opt_out_smart_routing = f.opt_out_smart_routing;
        let params = f.clearing_params;
        order.clearing_account = params.clearing_account;
        order.clearing_intent = params.clearing_intent;
        order.not_held = f.not_held;
        if let DeltaNeutral::Some(c) = f.c_delta_neutral {
            contract.delta_neutral_contract = Some(c);
        }
        if let Some((strategy, params)) = f.algo {
            order.algo_strategy = strategy;
            order.algo_params = params;
        }
        order.solicited = f.solicited;
        let comm = f.what_if_info_and_commission;
        order.what_if = comm.what_if;
        state.status = comm.s_status;
        state.init_margin_before = comm.s_init_margin_before;
        state.maint_margin_before = comm.s_maint_margin_before;
        state.equity_with_loan_before = comm.s_equity_with_loan_before;
        state.init_margin_change = comm.s_init_margin_change;
        state.maint_margin_change = comm.s_maint_margin_change;
        state.equity_with_loan_change = comm.s_equity_with_loan_change;
        state.init_margin_after = comm.s_init_margin_after;
        state.maint_margin_after = comm.s_maint_margin_after;
        state.equity_with_loan_after = comm.s_equity_with_loan_after;
        state.commission = comm.s_commission;
        state.min_commission = comm.s_min_commission;
        state.max_commission = comm.s_max_commission;
        state.commission_currency = comm.s_commission_currency;
        state.warning_text = comm.s_warning_text;
        let flags = f.vol_randomize_flags;
        order.randomize_size = flags.randomize_size;
        order.randomize_price = flags.randomize_price;
        // TODO only if PEG_BENCH: peg_to_bench_params: PegToBenchParams
        let conditions = f.conditions;
        order.conditions = conditions.conditions;
        // TODO if conditions.len() > 0
        //   conditionsIgnoreRth
        //   conditionsCancelOrder
        let params = f.adjusted_order_params;
        order.adjusted_order_type = params.adjusted_order_type;
        order.trigger_price = params.trigger_price;
        order.trail_stop_price = params.stop_price_and_lmt_price_offset.trail_stop_price;
        order.lmt_price_offset = params.stop_price_and_lmt_price_offset.lmt_price_offset;
        order.adjusted_stop_price = params.adjusted_stop_price;
        order.adjusted_stop_limit_price = params.adjusted_stop_limit_price;
        order.adjusted_trailing_amount = params.adjusted_trailing_amount;
        order.adjustable_trailing_unit = params.adjustable_trailing_unit;
        order.soft_dollar_tier = f.soft_dollar_tier;
        order.cash_qty = f.cash_qty;
        order.dont_use_auto_price_for_hedge = f.dont_use_auto_price_for_hedge;
        order.is_oms_container = f.is_oms_container;
        order.discretionary_up_to_limit_price = f.discretionary_up_to_limit_price;
        order.use_price_mgmt_algo = f.use_price_mgmt_algo;

        OpenOrder { contract, order, state }
    }
}

#[derive(Deserialize)]
struct ContractFields {
    conid: i32,
    symbol: String,
    sec_type: String,
    last_trade_date_or_contract_month: String,
    strike: f64,
    right: Right,
    multiplier: String,
    exchange: String,
    currency: String,
    local_symbol: String,
    trading_class: String,
}

#[derive(Deserialize)]
struct OrderBasic {
    action: Action,
    total_quantity: f64,
    order_type: OrderType,
    lmt_price: Option<f64>,
    aux_price: Option<f64>,
    tif: TimeInForce,
    oca_group: String,
    account: String,
    open_close: OpenClose,
    origin: Origin,
    order_ref: String,
}

#[derive(Deserialize)]
struct FAParams {
    group: String,
    method: String,
    percentage: String,
    profile: String,
}

#[derive(Deserialize)]
struct ShortSaleParams {
    short_sale_slot: ShortSaleSlot,
    designated_location: String,
    exempt_code: i32,
}

#[derive(Deserialize)]
struct BoxOrderParams {
    starting_price: Option<f64>,
    stock_ref_price: Option<f64>,
    delta: Option<f64>,
}

#[derive(Deserialize)]
struct PegToStkOrVolOrderParams {
    stock_range_lower: Option<f64>,
    stock_range_upper: Option<f64>,
}

/// O should be either OpenOrderAttribs or ()
#[derive(Deserialize)]
struct VolOrderParams<O> {
    volatility: Option<f64>,
    volatility_type: VolatilityType,
    _delta_neutral_order_type: DeltaNeutralOrderType, //TODO: we can only decode None for now
    delta_neutral_aux_price: Option<f64>,
    delta_neutral_params: DeltaNeutralParams<O>,
    continuous_update: bool,
    reference_price_type: ReferencePriceType,
}

// TODO: handle other order types. Note that if "" is sent,
// `delta_neutral_params` will not be sent
#[derive(Deserialize)]
enum DeltaNeutralOrderType {
    None
}

/// O should be either OpenOrderAttribs or ()
#[derive(Deserialize)]
struct DeltaNeutralParams<O> {
    delta_neutral_con_id: Option<i32>, // TODO 0 default
    open_order_attribs: O,
    delta_neutral_short_sale: Option<bool>,
    delta_neutral_short_sale_slot: ShortSaleSlot,
    delta_neutral_designated_location: String,
}

#[derive(Deserialize)]
struct OpenOrderAttribs {
    settling_firm: String,
    clearing_account: String,
    clearing_intent: ClearingIntent,
    open_close: OpenClose,
}

#[derive(Deserialize)]
struct TrailParams {
    trail_stop_price: Option<f64>,
    trailing_percent: Option<f64>,
}

#[derive(Deserialize)]
struct BasisPoints {
    basis_points: Option<f64>,
    basis_points_type: Option<i32>,
}

#[derive(Deserialize)]
struct ComboLegs {
    c_combo_legs_descrip: String,
    c_combo_legs: Vec<ComboLeg>,
    order_combo_legs: Vec<OrderComboLeg>,
}

#[derive(Deserialize)]
struct ScaleOrderParams {
    scale_init_level_size: Option<i32>,
    scale_subs_level_size: Option<i32>,
    _scale_price_increment: EmptyString, // TODO Option<f64>,

    /* TODO: requires manual deserialize implementation
    if scalePriceIncrement > 0.0 && scalePriceIncrement != Double.MAX_VALUE {
        scalePriceAdjustValue: Option<f64>,
        scalePriceAdjustInterval: Option<i32>,
        scaleProfitOffset: Option<f64>,
        scaleAutoReset: bool,
        scaleInitPosition: Option<i32>,
        scaleInitFillQty: Option<i32>,
        scaleRandomPercent: bool,
    } */
}

#[derive(Deserialize)]
struct ClearingParams {
    clearing_account: String,
    clearing_intent: ClearingIntent,
}

#[derive(Deserialize)]
enum DeltaNeutral {
    #[serde(rename="0")] None,
    #[serde(rename="1")] Some(DeltaNeutralContract),
}

#[derive(Deserialize)]
struct WhatIfInfoAndCommission {
    what_if: bool,
    s_status: String,

    s_init_margin_before: String,
    s_maint_margin_before: String,
    s_equity_with_loan_before: String,
    s_init_margin_change: String,
    s_maint_margin_change: String,
    s_equity_with_loan_change: String,

    s_init_margin_after: String,
    s_maint_margin_after: String,
    s_equity_with_loan_after: String,
    s_commission: Option<f64>,
    s_min_commission: Option<f64>,
    s_max_commission: Option<f64>,
    s_commission_currency: String,
    s_warning_text: String,
}

#[derive(Deserialize)]
struct VolRandomizeFlags {
    randomize_size: bool,
    randomize_price: bool,
}

// TODO: order_type == OrderType.PEG_BENCH
#[allow(dead_code)]
#[derive(Deserialize)]
struct PegToBenchParams {
    reference_contract_id: i32,
    is_pegged_change_amount_decrease: bool,
    pegged_change_amount: f64,
    reference_change_amount: f64,
    reference_exchange_id: String,
}

#[derive(Deserialize)]
struct Conditions {
    conditions: Vec<OrderCondition>,
    // TODO if conditions.len() > 0 {
    //conditionsIgnoreRth: bool,
    //conditionsCancelOrder: bool,
}

#[derive(Deserialize)]
struct AdjustedOrderParams {
    adjusted_order_type: String, // TODO: "None" or OrderType,
    trigger_price: Option<f64>,
    stop_price_and_lmt_price_offset: StopPriceAndLmtPriceOffset,
    adjusted_stop_price: Option<f64>,
    adjusted_stop_limit_price: Option<f64>,
    adjusted_trailing_amount: Option<f64>,
    adjustable_trailing_unit: i32,
}

#[derive(Deserialize)]
struct StopPriceAndLmtPriceOffset {
    trail_stop_price: Option<f64>,
    lmt_price_offset: Option<f64>,
}

#[derive(Deserialize)]
enum EmptyString {
    #[serde(rename="")] None,
}

/// If string is empty, don't read any other fields.
/// Otherwise, keep string and read type
enum OptionalType<T> {
    None,
    Some(String, T),
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for OptionalType<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::fmt;
        use std::marker::PhantomData;

        use serde::de::{Visitor, SeqAccess, MapAccess};

        struct OptionalTypeVisitor<T> {
            marker: PhantomData<fn() -> OptionalTypeVisitor<T>>,
            m: PhantomData<T>,
        }

        impl<'de, T: Deserialize<'de>> Visitor<'de> for OptionalTypeVisitor<T> {
            type Value = OptionalType<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("??#?$@?$#?@#$?@struct Duration")
            }

            fn visit_seq<V: SeqAccess<'de>>(self, mut seq: V) -> Result<Self::Value, V::Error> {
                let kind: String = seq.next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

                if kind.len() == 0 {
                    return Ok(OptionalType::None);
                }

                let t: T = seq.next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

                Ok(OptionalType::Some(kind, t))
            }

            fn visit_map<V: MapAccess<'de>>(self, _: V) -> Result<Self::Value, V::Error> {
                unimplemented!()
            }
        }

        const FIELDS: &'static [&'static str] = &[""; 2];
        deserializer.deserialize_struct("OptionalType", FIELDS, OptionalTypeVisitor { marker: PhantomData, m: PhantomData })
    }
}
