use super::*;

fn order_book() -> MarketOrderBook {
    MarketOrderBook::new(
        vec![MarketQuote::new(100.0, 1.5).unwrap()],
        vec![MarketQuote::new(101.0, 2.5).unwrap()],
    )
    .unwrap()
}

fn quote(price: f64, quantity: f64) -> MarketQuote {
    MarketQuote::new(price, quantity).unwrap()
}

#[test]
fn bids_and_asks_are_sorted_by_price() {
    let order_book = MarketOrderBook::new(
        vec![
            MarketQuote::new(99.0, 1.0).unwrap(),
            MarketQuote::new(101.0, 2.0).unwrap(),
            MarketQuote::new(100.0, 3.0).unwrap(),
        ],
        vec![
            MarketQuote::new(103.0, 1.0).unwrap(),
            MarketQuote::new(102.0, 2.0).unwrap(),
            MarketQuote::new(104.0, 3.0).unwrap(),
        ],
    )
    .unwrap();

    assert_eq!(
        order_book.bids(),
        [quote(101.0, 2.0), quote(100.0, 3.0), quote(99.0, 1.0)]
    );
    assert_eq!(
        order_book.asks(),
        [quote(102.0, 2.0), quote(103.0, 1.0), quote(104.0, 3.0)]
    );
}

#[test]
fn serde_json_round_trip() {
    let order_book = order_book();
    let json = serde_json::to_string(&order_book).unwrap();
    let deserialized: MarketOrderBook = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.bids, [quote(100.0, 1.5)]);
    assert_eq!(deserialized.asks, [quote(101.0, 2.5)]);
}

#[test]
fn deserialization_sorts_bids_and_asks() {
    let json = r#"{
        "bids": [{"price": 99.0, "quantity": 1.0}, {"price": 101.0, "quantity": 2.0}],
        "asks": [{"price": 103.0, "quantity": 1.0}, {"price": 102.0, "quantity": 2.0}]
    }"#;
    let deserialized: MarketOrderBook = serde_json::from_str(json).unwrap();

    assert_eq!(deserialized.bids(), [quote(101.0, 2.0), quote(99.0, 1.0)]);
    assert_eq!(deserialized.asks(), [quote(102.0, 2.0), quote(103.0, 1.0)]);
}

#[test]
fn deserialization_rejects_invalid_quotes() {
    let json = r#"{
        "bids": [{"price": 0.0, "quantity": 1.0}],
        "asks": [{"price": 102.0, "quantity": -1.0}]
    }"#;

    assert!(serde_json::from_str::<MarketOrderBook>(json).is_err());
}

#[test]
fn new_rejects_crossed_bids_and_asks() {
    let cases = [(100.0, 100.0), (101.0, 100.0), (100.0, 99.0)];

    for (bid_price, ask_price) in cases {
        let bid = quote(bid_price, 1.0);
        let ask = quote(ask_price, 1.0);
        assert!(MarketOrderBook::new(vec![bid], vec![ask]).is_err());
    }
}

#[test]
fn new_accepts_an_order_book_with_a_missing_side() {
    MarketOrderBook::new(vec![quote(100.0, 1.0)], Vec::new()).unwrap();
    MarketOrderBook::new(Vec::new(), vec![quote(101.0, 1.0)]).unwrap();
    MarketOrderBook::new(Vec::new(), Vec::new()).unwrap();
}

#[test]
fn deserialization_rejects_crossed_bids_and_asks() {
    let json = r#"{
        "bids": [{"price": 101.0, "quantity": 1.0}],
        "asks": [{"price": 100.0, "quantity": 2.0}]
    }"#;

    assert!(serde_json::from_str::<MarketOrderBook>(json).is_err());
}
