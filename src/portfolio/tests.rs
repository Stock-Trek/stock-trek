use super::*;
use stock_trek_types::cex::{activation::Activation, time_in_force::TimeInForce};

fn portfolio_with(
    order_request: OrderRequest<AssetId, f64>,
    filled_base_quantity: f64,
    filled_quote_quantity: f64,
) -> Portfolio {
    let mut builder = PortfolioBuilder::new();
    builder
        .asset_count(
            CexId::Binance,
            AccountId::new("account"),
            AssetId::Bitcoin,
            100.0,
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

fn limit_sell(quantity: Quantity<f64>, limit_price: f64) -> OrderRequest<AssetId, f64> {
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
fn asset_iterators_report_balances_for_each_account() {
    let account_a = AccountId::new("a");
    let account_b = AccountId::new("b");
    let mut builder = PortfolioBuilder::new();
    builder
        .asset_count(CexId::Binance, account_a.clone(), AssetId::Bitcoin, 2.0)
        .asset_count(CexId::Binance, account_a.clone(), AssetId::TetherUSD, 3.0)
        .asset_count(CexId::Binance, account_b.clone(), AssetId::Bitcoin, 4.0);
    let portfolio = builder.build();

    let balances: std::collections::HashMap<_, _> = portfolio
        .assets_iter()
        .map(|(cex_id, account_id, asset_id, count)| {
            ((*cex_id, account_id.clone(), *asset_id), *count)
        })
        .collect();

    assert_eq!(balances.len(), 3);
    assert_eq!(
        balances.get(&(CexId::Binance, account_a.clone(), AssetId::Bitcoin)),
        Some(&2.0)
    );
    assert_eq!(
        balances.get(&(CexId::Binance, account_a.clone(), AssetId::TetherUSD)),
        Some(&3.0)
    );
    assert_eq!(
        balances.get(&(CexId::Binance, account_b.clone(), AssetId::Bitcoin)),
        Some(&4.0)
    );

    assert_eq!(
        portfolio
            .assets_in_cex_account_iter(&CexId::Binance, &account_a)
            .count(),
        2
    );
    assert_eq!(
        portfolio.asset_total_in_cex_account(&CexId::Binance, &account_b, &AssetId::Bitcoin,),
        4.0
    );
    assert_eq!(portfolio.asset_total(&AssetId::Bitcoin), 6.0);
}

#[test]
fn pending_order_count_helpers_agree_with_their_iterators() {
    let account_a = AccountId::new("a");
    let account_b = AccountId::new("b");
    let mut builder = PortfolioBuilder::new();
    builder
        .pending_order(
            CexId::Binance,
            account_a.clone(),
            limit_sell(Quantity::OfBase(1.0), 100.0),
            0.0,
            0.0,
        )
        .pending_order(
            CexId::Binance,
            account_b.clone(),
            limit_sell(Quantity::OfBase(1.0), 100.0),
            0.0,
            0.0,
        );
    let portfolio = builder.build();

    assert_eq!(portfolio.pending_orders_count(), 2);
    assert_eq!(portfolio.pending_orders_with_tag_count(&Tag::new("tag")), 2);
    assert_eq!(
        portfolio.pending_orders_with_tag_count(&Tag::new("other")),
        0
    );
    assert_eq!(
        portfolio.pending_orders_in_cex_account_count(&CexId::Binance, &account_a),
        1
    );
}

#[test]
fn reserved_quantity_is_clamped_when_filled_exceeds_ordered() {
    let portfolio = portfolio_with(limit_sell(Quantity::OfBase(10.0), 100.0), 15.0, 0.0);

    assert_eq!(portfolio.asset_reserved(&AssetId::Bitcoin), 0.0);
    assert_eq!(
        portfolio.asset_available(&AssetId::Bitcoin),
        portfolio.asset_total(&AssetId::Bitcoin)
    );
}

#[test]
fn zero_price_quote_limited_sell_reserves_nothing() {
    let portfolio = portfolio_with(limit_sell(Quantity::OfQuote(1_000.0), 0.0), 0.0, 0.0);

    assert_eq!(portfolio.asset_reserved(&AssetId::Bitcoin), 0.0);
}

#[test]
fn partial_fill_reserves_remaining_quantity() {
    let portfolio = portfolio_with(limit_sell(Quantity::OfBase(10.0), 100.0), 4.0, 0.0);

    assert_eq!(portfolio.asset_reserved(&AssetId::Bitcoin), 6.0);
}

#[test]
fn market_orders_reserve_quantities_are_clamped() {
    let market_buy = OrderRequest::MarketBuy {
        base: AssetId::Bitcoin,
        quote: AssetId::TetherUSD,
        activation: Activation::Immediate,
        quote_quantity: 100.0,
        tag: Tag::new("tag"),
    };
    let portfolio = portfolio_with(market_buy, 0.0, 150.0);
    assert_eq!(portfolio.asset_reserved(&AssetId::TetherUSD), 0.0);

    let market_sell = OrderRequest::MarketSell {
        base: AssetId::Bitcoin,
        quote: AssetId::TetherUSD,
        activation: Activation::Immediate,
        base_quantity: 10.0,
        tag: Tag::new("tag"),
    };
    let portfolio = portfolio_with(market_sell, 15.0, 0.0);
    assert_eq!(portfolio.asset_reserved(&AssetId::Bitcoin), 0.0);
}

#[test]
fn asset_available_is_clamped_when_reservations_exceed_balance() {
    let portfolio = portfolio_with(limit_sell(Quantity::OfBase(150.0), 100.0), 0.0, 0.0);

    assert_eq!(portfolio.asset_reserved(&AssetId::Bitcoin), 150.0);
    assert_eq!(portfolio.asset_available(&AssetId::Bitcoin), 0.0);
    assert_eq!(
        portfolio.asset_available_in_cex_account(
            &CexId::Binance,
            &AccountId::new("account"),
            &AssetId::Bitcoin,
        ),
        0.0
    );
}

#[test]
fn pending_orders_are_grouped_by_the_order_request_tag() {
    let mut builder = PortfolioBuilder::new();
    builder.pending_order(
        CexId::Binance,
        AccountId::new("account"),
        limit_sell(Quantity::OfBase(10.0), 100.0),
        0.0,
        0.0,
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
