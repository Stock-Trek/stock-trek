use super::*;

use crate::market_data::market::MarketBuilder;

fn market() -> Market {
    MarketBuilder::new(0.000_001, 0.01, 1.0).build().unwrap()
}

#[test]
fn markets_are_indexed_by_cex_base_and_quote() {
    let mut builder = SignalContextBuilder::new();
    builder.cex_market(
        CexId::Binance,
        AssetId::Bitcoin,
        AssetId::TetherUSD,
        market(),
    );
    builder.cex_market(
        CexId::Binance,
        AssetId::Ethereum,
        AssetId::TetherUSD,
        market(),
    );
    let context = builder.build().unwrap();

    assert!(
        context
            .cex_market_for(CexId::Binance, AssetId::Bitcoin, AssetId::TetherUSD)
            .is_some()
    );
    assert!(
        context
            .cex_market_for(CexId::Binance, AssetId::Ethereum, AssetId::TetherUSD)
            .is_some()
    );
    assert!(
        context
            .cex_market_for(CexId::Binance, AssetId::Bitcoin, AssetId::USDCoin)
            .is_none()
    );
    assert_eq!(
        context
            .cex_markets_for(AssetId::Bitcoin, AssetId::TetherUSD)
            .count(),
        1
    );
}

#[test]
fn constructors_reject_duplicate_keys() {
    let by_quote = CexMarketDataByQuoteContext::new([
        (AssetId::TetherUSD, market()),
        (AssetId::TetherUSD, market()),
    ]);
    assert!(by_quote.is_err());

    let by_quote = CexMarketDataByQuoteContext::new([(AssetId::TetherUSD, market())]).unwrap();
    let by_base = CexMarketDataByBaseContext::new([
        (AssetId::Bitcoin, by_quote.clone()),
        (AssetId::Bitcoin, by_quote.clone()),
    ]);
    assert!(by_base.is_err());

    let by_base = CexMarketDataByBaseContext::new([(AssetId::Bitcoin, by_quote)]).unwrap();
    let context =
        SignalContext::new([(CexId::Binance, by_base.clone()), (CexId::Binance, by_base)]);
    assert!(context.is_err());
}

#[test]
fn builder_rejects_duplicate_markets() {
    let mut builder = SignalContextBuilder::new();
    builder.cex_market(
        CexId::Binance,
        AssetId::Bitcoin,
        AssetId::TetherUSD,
        market(),
    );
    builder.cex_market(
        CexId::Binance,
        AssetId::Bitcoin,
        AssetId::TetherUSD,
        market(),
    );

    assert!(builder.build().is_err());
}
