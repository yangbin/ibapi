use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, SmartDefault)]
pub enum TickByTickType {
    #[default] None, Last, AllLast, BidAsk, MidPoint,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ComboParam {
    NonGuaranteed, PriceCondConid, CondPriceMax, CondPriceMin, ChangeToMktTime1, ChangeToMktTime2, DiscretionaryPct, DontLeginNext, LeginPrio, MaxSegSize,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AlgoParam {
    startTime, endTime, allowPastEndTime, maxPctVol, pctVol, strategyType, noTakeLiq, riskAversion, forceCompletion, displaySize, getDone, noTradeAhead, useOddLots,
    componentSize, timeBetweenOrders, randomizeTime20, randomizeSize55, giveUp, catchUp, waitForFill
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, SmartDefault)]
pub enum AlgoStrategy {
    #[default] None, Vwap, Twap, ArrivalPx, DarkIce, PctVol, AD,
}

impl AlgoStrategy {
    pub fn params(&self) -> &[AlgoParam] {
        use AlgoParam::*;
        use AlgoStrategy::*;

        match self {
            None => &[],
            Vwap => &[startTime, endTime, maxPctVol, noTakeLiq, getDone, noTradeAhead, useOddLots],
            Twap => &[startTime, endTime, allowPastEndTime, strategyType],
            ArrivalPx => &[startTime, endTime, allowPastEndTime, maxPctVol, riskAversion, forceCompletion],
            DarkIce => &[startTime, endTime, allowPastEndTime, displaySize],
            PctVol => &[startTime, endTime, pctVol, noTakeLiq],
            AD => &[startTime, endTime, componentSize, timeBetweenOrders, randomizeTime20, randomizeSize55, giveUp, catchUp, waitForFill],
        }
    }
}

/// `hedge_param` is associated data of variant
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, SmartDefault)]
pub enum HedgeType {
    #[default]
    #[serde(rename="")] None,
    #[serde(rename="D")] Delta(String),
    #[serde(rename="B")] Beta(String),
    #[serde(rename="F")] Fx(String),
    #[serde(rename="P")] Pair(String),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, SmartDefault)]
pub enum Right {
    #[default]
    #[serde(rename="", alias="?")] None,
    #[serde(rename="P")] Put,
    #[serde(rename="C")] Call,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, SmartDefault)]
pub enum VolatilityType {
    #[default]
    #[serde(rename="0", alias="2147483647")] None,
    #[serde(rename="1")] Daily,
    #[serde(rename="2")] Annual,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, SmartDefault)]
pub enum ReferencePriceType {
    #[default]
    //#[serde(rename="2147483647")] None,
    #[serde(rename="0")] None,
    #[serde(rename="1")] Midpoint,
    #[serde(rename="2")] BidOrAsk,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, SmartDefault)]
pub enum TriggerMethod {
    #[default]
    #[serde(rename="0")] Default,
    #[serde(rename="1")] DoubleBidAsk,
    #[serde(rename="2")] Last,
    #[serde(rename="3")] DoubleLast,
    #[serde(rename="4")] BidAsk,
    #[serde(rename="7")] LastOrBidAsk,
    #[serde(rename="8")] Midpoint,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Action {
    #[serde(rename="")] None,
    BUY, SELL, SSHORT,
}

impl Default for Action { fn default() -> Action { Action::None } }

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, SmartDefault)]
pub enum Rule80A {
    #[default]
    #[serde(rename="", alias="0")] None, // "0" being sent in the field
    #[serde(rename="I")] Individual,
    #[serde(rename="A")] Agency,
    #[serde(rename="W")] AgentOtherMember,
    #[serde(rename="J")] IndividualPTIA,
    #[serde(rename="U")] AgencyPTIA,
    #[serde(rename="M")] AgentOtherMemberPTIA,
    #[serde(rename="K")] IndividualPT,
    #[serde(rename="Y")] AgencyPT,
    #[serde(rename="N")] AgentOtherMemberPT,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, SmartDefault)]
pub enum OcaType {
    #[default]
    #[serde(rename="0")] None,
    #[serde(rename="1")] CancelWithBlocking,
    #[serde(rename="2")] ReduceWithBlocking,
    #[serde(rename="3")] ReduceWithoutBlocking,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, SmartDefault)]
pub enum TimeInForce {
    #[default] DAY, GTC, OPG, IOC, GTD, GTT, AUC, FOK, GTX, DTC,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ExerciseType {
    None, Exercise, Lapse,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum FundamentalType {
    #[serde(rename="Company overview")] ReportSnapshot,
    #[serde(rename="Financial summary")] ReportsFinSummary,
    #[serde(rename="Financial ratios")] ReportRatios,
    #[serde(rename="Financial statements")] ReportsFinStatements,
    #[serde(rename="Analyst estimates")] RESC,
    #[serde(rename="Company calendar")] CalendarReport,
    #[serde(rename="Company ownership")] ReportsOwnership,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum WhatToShow {
    TRADES, MIDPOINT, BID, ASK, // << only these are valid for real-time bars
    BID_ASK, HISTORICAL_VOLATILITY, OPTION_IMPLIED_VOLATILITY, YIELD_ASK, YIELD_BID, YIELD_BID_ASK, YIELD_LAST, ADJUSTED_LAST
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum BarSize {
    #[serde(rename="1 secs")] _1_secs,
    #[serde(rename="5 secs")] _5_secs,
    #[serde(rename="10 secs")] _10_secs,
    #[serde(rename="15 secs")] _15_secs,
    #[serde(rename="30 secs")] _30_secs,
    #[serde(rename="1 min")] _1_min,
    #[serde(rename="2 mins")] _2_mins,
    #[serde(rename="3 mins")] _3_mins,
    #[serde(rename="5 mins")] _5_mins,
    #[serde(rename="10 mins")] _10_mins,
    #[serde(rename="15 mins")] _15_mins,
    #[serde(rename="20 mins")] _20_mins,
    #[serde(rename="30 mins")] _30_mins,
    #[serde(rename="1 hour")] _1_hour,
    #[serde(rename="4 hours")] _4_hours,
    #[serde(rename="1 day")] _1_day,
    #[serde(rename="1 week")] _1_week,
    #[serde(rename="1 month")] _1_month,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum DurationUnit {
    SECOND, DAY, WEEK, MONTH, YEAR,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum DeepType {
    #[serde(rename="0")] INSERT,
    #[serde(rename="1")] UPDATE,
    #[serde(rename="2")] DELETE,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum DeepSide {
    #[serde(rename="0")] SELL,
    #[serde(rename="1")] BUY,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum NewsType {
    #[serde(rename="0")] UNKNOWN,
    #[serde(rename="1")] BBS,
    #[serde(rename="2")] LIVE_EXCH,
    #[serde(rename="3")] DEAD_EXCH,
    #[serde(rename="4")] HTML,
    #[serde(rename="5")] POPUP_TEXT,
    #[serde(rename="6")] POPUP_HTML,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum FADataType {
    #[serde(rename="0")] UNUSED,
    #[serde(rename="1")] GROUPS,
    #[serde(rename="2")] PROFILES,
    #[serde(rename="3")] ALIASES,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SecIdType {
    #[serde(rename="")] None,
    CUSIP, SEDOL, ISIN, RIC,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SecType {
    #[serde(rename="")] None,
    STK, OPT, FUT, CONTFUT, CASH, BOND, CFD, FOP, WAR, IOPT, FWD, BAG, IND, BILL, FUND, FIXED, SLB, NEWS, CMDTY, BSK, ICU, ICS,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum MktDataType {
    #[serde(rename="0")] Unknown,
    #[serde(rename="1")] Realtime,
    #[serde(rename="2")] Frozen,
    #[serde(rename="3")] Delayed,
    #[serde(rename="4")] DelayedFrozen
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Method {
    #[serde(rename="")] None,
    EqualQuantity, AvailableEquity, NetLiq, PctChange,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum UsePriceMgmtAlgo {
    #[serde(rename="2147483647")] None,
    #[serde(rename="0")] NotUse,
    #[serde(rename="1")] Use,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TagValue {
    tag: String,
    value: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SoftDollarTier {
    pub name: String,
    pub value: String,
    pub display_name: String,
}
