# Enum varient to String

Tags: code

# Strum for OrderTypes

Use the strum libaray 

[https://docs.rs/strum/latest/strum/derive.EnumString.html](https://docs.rs/strum/latest/strum/derive.EnumString.html)

just add the `EnumString` macro above the emum.

```toml
[package]
name = "f_strum"
version = "0.1.0"
edition = "2021"

[dependencies]
strum = "0.25.0"
strum_macros = "0.25.0"
```

```rust
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
```