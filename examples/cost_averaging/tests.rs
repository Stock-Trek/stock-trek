use super::*;
use stock_trek::markets::{Market, MarketBuilder, TimestampMillis};

fn market(asks: &[f64]) -> Market {
    let mut builder = MarketBuilder::new(0.000_001, 0.01, 0.000_1);
    for (index, ask) in asks.iter().enumerate() {
        builder.tick(
            TimestampMillis::from(index as u64),
            (ask - 1.0, 1.0),
            (*ask, 1.0),
            (*ask, 1.0),
        );
    }
    builder.build()
}

fn context(markets: Vec<(CexId, Market)>) -> SignalContext {
    let mut builder = SignalContextBuilder::new();
    for (cex, market) in markets {
        builder.cex_market(cex, AssetId::Bitcoin, AssetId::TetherUSD, market);
    }
    builder.build()
}

#[test]
fn writes_the_account_even_without_markets() {
    let algorithm = CostAveraging::default();
    let signals = algorithm.signals(&context(vec![])).unwrap();

    assert_eq!(
        signals.read(&algorithm.key_account).unwrap(),
        AccountId::new(CEX_ACCOUNT_ID)
    );
    assert!(!signals.read(&algorithm.key_market_exists).unwrap());
}

#[test]
fn uses_the_latest_ask_and_handles_empty_markets() {
    let algorithm = CostAveraging::default();
    let latest = algorithm
        .signals(&context(vec![(CexId::Binance, market(&[300.0, 200.0]))]))
        .unwrap();
    let empty = algorithm
        .signals(&context(vec![(CexId::Binance, market(&[]))]))
        .unwrap();

    assert!(latest.read(&algorithm.key_market_exists).unwrap());
    assert_eq!(latest.read(&algorithm.key_cheapest_price).unwrap(), 200.0);
    assert!(!empty.read(&algorithm.key_market_exists).unwrap());
}

#[test]
fn skips_non_finite_asks() {
    let algorithm = CostAveraging::default();
    let signals = algorithm
        .signals(&context(vec![(CexId::Binance, market(&[f64::NAN]))]))
        .unwrap();

    assert!(!signals.read(&algorithm.key_market_exists).unwrap());
}
