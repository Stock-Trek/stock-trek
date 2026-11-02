use super::*;
use crate::market_data::market::MarketBuilder;

fn market() -> Market {
    MarketBuilder::new(0.000_001, 0.01, 1.0).build()
}

#[test]
fn markets_are_indexed_by_cex_base_and_quote() {
    let mut builder = SignalContextBuilder::new();
    builder
        .cex_market(
            CexId::Binance,
            AssetId::Bitcoin,
            AssetId::TetherUSD,
            market(),
        )
        .cex_market(
            CexId::Binance,
            AssetId::Ethereum,
            AssetId::TetherUSD,
            market(),
        );
    let context = builder.build();

    assert!(
        context
            .cex_market_for(&CexId::Binance, &AssetId::Bitcoin, &AssetId::TetherUSD)
            .is_some()
    );
    assert!(
        context
            .cex_market_for(&CexId::Binance, &AssetId::Ethereum, &AssetId::TetherUSD)
            .is_some()
    );
    assert!(
        context
            .cex_market_for(&CexId::Binance, &AssetId::Bitcoin, &AssetId::USDCoin)
            .is_none()
    );
    assert_eq!(
        context
            .cex_markets_for(AssetId::Bitcoin, AssetId::TetherUSD)
            .count(),
        1
    );
}
