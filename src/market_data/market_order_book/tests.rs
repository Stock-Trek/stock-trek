use super::*;

fn order_book() -> MarketOrderBook {
    MarketOrderBook::new(
        vec![MarketQuote::new(100.0, 1.5)],
        vec![MarketQuote::new(101.0, 2.5)],
    )
}

#[test]
fn bids_and_asks_are_sorted_by_price() {
    let order_book = MarketOrderBook::new(
        vec![
            MarketQuote::new(99.0, 1.0),
            MarketQuote::new(101.0, 2.0),
            MarketQuote::new(100.0, 3.0),
        ],
        vec![
            MarketQuote::new(103.0, 1.0),
            MarketQuote::new(102.0, 2.0),
            MarketQuote::new(104.0, 3.0),
        ],
    );

    assert_eq!(
        order_book.bid_price_quantities(),
        vec![(101.0, 2.0), (100.0, 3.0), (99.0, 1.0)]
    );
    assert_eq!(
        order_book.ask_price_quantities(),
        vec![(102.0, 2.0), (103.0, 1.0), (104.0, 3.0)]
    );
}

#[test]
fn serde_json_round_trip() {
    let order_book = order_book();
    let json = serde_json::to_string(&order_book).unwrap();
    let deserialized: MarketOrderBook = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.bids.len(), 1);
    assert_eq!(deserialized.bids[0].price(), 100.0);
    assert_eq!(deserialized.bids[0].quantity(), 1.5);
    assert_eq!(deserialized.asks.len(), 1);
    assert_eq!(deserialized.asks[0].price(), 101.0);
    assert_eq!(deserialized.asks[0].quantity(), 2.5);
    assert_eq!(deserialized.bid_price_quantities(), vec![(100.0, 1.5)]);
    assert_eq!(deserialized.ask_price_quantities(), vec![(101.0, 2.5)]);
}

#[test]
fn deserialization_sorts_bids_and_asks() {
    let json = r#"{
        "bids": [{"price": 99.0, "quantity": 1.0}, {"price": 101.0, "quantity": 2.0}],
        "asks": [{"price": 103.0, "quantity": 1.0}, {"price": 102.0, "quantity": 2.0}]
    }"#;
    let deserialized: MarketOrderBook = serde_json::from_str(json).unwrap();

    assert_eq!(
        deserialized.bid_price_quantities(),
        vec![(101.0, 2.0), (99.0, 1.0)]
    );
    assert_eq!(
        deserialized.ask_price_quantities(),
        vec![(102.0, 2.0), (103.0, 1.0)]
    );
}
