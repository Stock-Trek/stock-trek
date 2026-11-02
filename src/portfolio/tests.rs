use super::*;
use stock_trek_types::cex::{activation::Activation, time_in_force::TimeInForce};

fn portfolio_with(
    order_request: OrderRequest<AssetId, Decimal>,
    filled_base_quantity: Decimal,
    filled_quote_quantity: Decimal,
) -> Portfolio {
    let mut builder = PortfolioBuilder::new();
    builder
        .asset_count(
            CexId::Binance,
            AccountId::new("account"),
            AssetId::Bitcoin,
            Decimal::from(100),
        )
        .pending_order(
            CexId::Binance,
            AccountId::new("account"),
            order_request,
            filled_base_quantity,
            filled_quote_quantity,
        );
    builder.build()
}

fn limit_sell(quantity: Quantity<Decimal>, limit_price: Decimal) -> OrderRequest<AssetId, Decimal> {
    OrderRequest::Limit {
        base: AssetId::Bitcoin,
        quote: AssetId::TetherUSD,
        side: Side::Sell,
        activation: Activation::Immediate,
        limit_price,
        time_in_force: TimeInForce::GoodTillCancelled,
        quantity,
        tag: Tag::new("tag"),
    }
}

#[test]
fn reserved_quantity_is_clamped_when_filled_exceeds_ordered() {
    let portfolio = portfolio_with(
        limit_sell(Quantity::OfBase(Decimal::from(10)), Decimal::from(100)),
        Decimal::from(15),
        Decimal::ZERO,
    );

    assert_eq!(portfolio.asset_reserved(&AssetId::Bitcoin), Decimal::ZERO);
    assert_eq!(
        portfolio.asset_available(&AssetId::Bitcoin),
        portfolio.asset_total(&AssetId::Bitcoin)
    );
}

#[test]
fn zero_price_quote_limited_sell_reserves_nothing() {
    let portfolio = portfolio_with(
        limit_sell(Quantity::OfQuote(Decimal::from(1_000)), Decimal::ZERO),
        Decimal::ZERO,
        Decimal::ZERO,
    );

    assert_eq!(portfolio.asset_reserved(&AssetId::Bitcoin), Decimal::ZERO);
}

#[test]
fn partial_fill_reserves_remaining_quantity() {
    let portfolio = portfolio_with(
        limit_sell(Quantity::OfBase(Decimal::from(10)), Decimal::from(100)),
        Decimal::from(4),
        Decimal::ZERO,
    );

    assert_eq!(
        portfolio.asset_reserved(&AssetId::Bitcoin),
        Decimal::from(6)
    );
}

#[test]
fn market_orders_reserve_quantities_are_clamped() {
    let market_buy = OrderRequest::MarketBuy {
        base: AssetId::Bitcoin,
        quote: AssetId::TetherUSD,
        activation: Activation::Immediate,
        quote_quantity: Decimal::from(100),
        tag: Tag::new("tag"),
    };
    let portfolio = portfolio_with(market_buy, Decimal::ZERO, Decimal::from(150));
    assert_eq!(portfolio.asset_reserved(&AssetId::TetherUSD), Decimal::ZERO);

    let market_sell = OrderRequest::MarketSell {
        base: AssetId::Bitcoin,
        quote: AssetId::TetherUSD,
        activation: Activation::Immediate,
        base_quantity: Decimal::from(10),
        tag: Tag::new("tag"),
    };
    let portfolio = portfolio_with(market_sell, Decimal::from(15), Decimal::ZERO);
    assert_eq!(portfolio.asset_reserved(&AssetId::Bitcoin), Decimal::ZERO);
}

#[test]
fn asset_available_is_clamped_when_reservations_exceed_balance() {
    let portfolio = portfolio_with(
        limit_sell(Quantity::OfBase(Decimal::from(150)), Decimal::from(100)),
        Decimal::ZERO,
        Decimal::ZERO,
    );

    assert_eq!(
        portfolio.asset_reserved(&AssetId::Bitcoin),
        Decimal::from(150)
    );
    assert_eq!(portfolio.asset_available(&AssetId::Bitcoin), Decimal::ZERO);
    assert_eq!(
        portfolio.asset_available_in_cex_account(
            &CexId::Binance,
            &AccountId::new("account"),
            &AssetId::Bitcoin,
        ),
        Decimal::ZERO
    );
}

#[test]
fn pending_orders_are_grouped_by_the_order_request_tag() {
    let mut builder = PortfolioBuilder::new();
    builder.pending_order(
        CexId::Binance,
        AccountId::new("account"),
        limit_sell(Quantity::OfBase(Decimal::from(10)), Decimal::from(100)),
        Decimal::ZERO,
        Decimal::ZERO,
    );
    let portfolio = builder.build();

    assert_eq!(
        portfolio.pending_orders_in_cex_account_with_tag_count(
            &CexId::Binance,
            &AccountId::new("account"),
            &Tag::new("tag"),
        ),
        1
    );
    assert_eq!(
        portfolio.pending_orders_in_cex_account_with_tag_count(
            &CexId::Binance,
            &AccountId::new("account"),
            &Tag::new("other"),
        ),
        0
    );
}
