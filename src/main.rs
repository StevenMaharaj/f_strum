use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, PartialEq, EnumString)]
enum OrderType {
    Limit = 1,
    Market = 2,
    Stop = 3,
    StopLimit = 4,
    TrailingStop = 5,
    FillOrKill = 6,
    ImmediateOrCancel = 7,
    PostOnly = 8,
    Fill = 9,
    PartialFill = 10,
    Cancel = 11,
    CancelAll = 12,
    CancelReplace = 13,
    CancelGroup = 14,
    CancelAllGroup = 15,
    CancelReplaceGroup = 16,
}
fn main() {
    let order_variant = OrderType::from_str("Limit").unwrap();
    assert_eq!(OrderType::Limit, order_variant);
}
