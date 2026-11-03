use super::*;
use crate::pending_order::PendingOrder;
use stock_trek_types::cex::{
    account_id::AccountId, activation::Activation, asset_id::AssetId, cex_id::CexId,
    order_request::OrderRequest, quantity::Quantity, side::Side, tag::Tag,
    time_in_force::TimeInForce,
};

fn portfolio_with(
    order_request: OrderRequest<AssetId, f64>,
    filled_base_quantity: f64,
    filled_quote_quantity: f64,
) -> Portfolio {
    PortfolioBuilder::new()
        .asset_count(
            CexId::Binance,
            AccountId::new("account"),
            AssetId::Bitcoin,
            100.0,
        )
        .unwrap()
        .pending_order(
            CexId::Binance,
            AccountId::new("account"),
            order_request,
            filled_base_quantity,
            filled_quote_quantity,
        )
        .unwrap()
        .build()
}

fn limit_sell(quantity: Quantity<f64>, limit_price: f64) -> OrderRequest<AssetId, f64> {
    limit_sell_with_tag(quantity, limit_price, Tag::new("tag"))
}

fn limit_sell_with_tag(
    quantity: Quantity<f64>,
    limit_price: f64,
    tag: Tag,
) -> OrderRequest<AssetId, f64> {
    OrderRequest::Limit {
        base: AssetId::Bitcoin,
        quote: AssetId::TetherUSD,
        side: Side::Sell,
        activation: Activation::Immediate,
        limit_price,
        time_in_force: TimeInForce::GoodTillCancelled,
        quantity,
        tag,
    }
}

fn determinism_portfolio(reverse: bool) -> Portfolio {
    let mut entries = [("a", 1e16), ("b", 1.0), ("c", 1.0)];
    if reverse {
        entries.reverse();
    }

    let mut builder = PortfolioBuilder::new();
    for (account, count) in entries {
        builder = builder
            .asset_count(
                CexId::Binance,
                AccountId::new(account),
                AssetId::Bitcoin,
                count,
            )
            .unwrap()
            .pending_order(
                CexId::Binance,
                AccountId::new(account),
                limit_sell_with_tag(Quantity::OfBase(count), 100.0, Tag::new(account)),
                0.0,
                0.0,
            )
            .unwrap();
    }
    builder.build()
}

#[test]
fn totals_are_deterministic_across_insertion_order() {
    let forward = determinism_portfolio(false);
    let reverse = determinism_portfolio(true);

    assert_eq!(
        forward.asset_total(&AssetId::Bitcoin).to_bits(),
        reverse.asset_total(&AssetId::Bitcoin).to_bits()
    );
    assert_eq!(
        forward.asset_reserved(&AssetId::Bitcoin).to_bits(),
        reverse.asset_reserved(&AssetId::Bitcoin).to_bits()
    );
}

#[test]
fn asset_iterators_report_balances_for_each_account() {
    let account_a = AccountId::new("a");
    let account_b = AccountId::new("b");
    let portfolio = PortfolioBuilder::new()
        .asset_count(CexId::Binance, account_a.clone(), AssetId::Bitcoin, 2.0)
        .unwrap()
        .asset_count(CexId::Binance, account_a.clone(), AssetId::TetherUSD, 3.0)
        .unwrap()
        .asset_count(CexId::Binance, account_b.clone(), AssetId::Bitcoin, 4.0)
        .unwrap()
        .build();

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
    let portfolio = PortfolioBuilder::new()
        .pending_order(
            CexId::Binance,
            account_a.clone(),
            limit_sell(Quantity::OfBase(1.0), 100.0),
            0.0,
            0.0,
        )
        .unwrap()
        .pending_order(
            CexId::Binance,
            account_b.clone(),
            limit_sell(Quantity::OfBase(1.0), 100.0),
            0.0,
            0.0,
        )
        .unwrap()
        .build();

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
fn portfolio_builder_rejects_zero_price_quote_limited_sell() {
    let result = PortfolioBuilder::new().pending_order(
        CexId::Binance,
        AccountId::new("account"),
        limit_sell(Quantity::OfQuote(1_000.0), 0.0),
        0.0,
        0.0,
    );

    assert!(result.is_err());
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
fn asset_available_clamps_reservations_per_account() {
    let account_a = AccountId::new("a");
    let account_b = AccountId::new("b");
    let portfolio = PortfolioBuilder::new()
        .asset_count(CexId::Binance, account_a.clone(), AssetId::Bitcoin, 100.0)
        .unwrap()
        .pending_order(
            CexId::Binance,
            account_a.clone(),
            limit_sell(Quantity::OfBase(150.0), 100.0),
            0.0,
            0.0,
        )
        .unwrap()
        .asset_count(CexId::Binance, account_b.clone(), AssetId::Bitcoin, 100.0)
        .unwrap()
        .build();

    assert_eq!(portfolio.asset_reserved(&AssetId::Bitcoin), 150.0);
    assert_eq!(portfolio.asset_available(&AssetId::Bitcoin), 100.0);
    assert_eq!(
        portfolio.asset_available(&AssetId::Bitcoin),
        portfolio
            .cex_accounts()
            .map(
                |(cex_id, account_id, _)| portfolio.asset_available_in_cex_account(
                    cex_id,
                    account_id,
                    &AssetId::Bitcoin
                )
            )
            .sum::<f64>()
    );
}

#[test]
fn pending_orders_are_grouped_by_the_order_request_tag() {
    let portfolio = PortfolioBuilder::new()
        .pending_order(
            CexId::Binance,
            AccountId::new("account"),
            limit_sell(Quantity::OfBase(10.0), 100.0),
            0.0,
            0.0,
        )
        .unwrap()
        .build();

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

#[test]
fn cex_account_portfolio_rejects_invalid_asset_balances() {
    for count in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let asset_counts = std::collections::HashMap::from([(AssetId::Bitcoin, count)]);
        let pending_orders = std::iter::empty::<(Tag, Vec<crate::pending_order::PendingOrder>)>();
        assert!(CexAccountPortfolio::new(asset_counts, pending_orders).is_err());
    }
}

#[test]
fn cex_account_portfolio_accepts_non_negative_asset_balances() {
    let asset_counts =
        std::collections::HashMap::from([(AssetId::Bitcoin, 0.0), (AssetId::TetherUSD, 100.0)]);

    let pending_orders = std::iter::empty::<(Tag, Vec<crate::pending_order::PendingOrder>)>();
    let portfolio = CexAccountPortfolio::new(asset_counts, pending_orders).unwrap();

    assert_eq!(portfolio.asset_count(&AssetId::Bitcoin), 0.0);
    assert_eq!(portfolio.asset_count(&AssetId::TetherUSD), 100.0);
}

#[test]
fn cex_account_portfolio_accepts_pending_orders_grouped_by_their_embedded_tag() {
    let order = PendingOrder::new(limit_sell(Quantity::OfBase(1.0), 100.0), 0.0, 0.0).unwrap();

    let portfolio =
        CexAccountPortfolio::new([(AssetId::Bitcoin, 10.0)], [(Tag::new("tag"), vec![order])])
            .unwrap();

    assert_eq!(
        portfolio
            .pending_orders_with_tag(&Tag::new("tag"))
            .map(|orders| orders.len()),
        Some(1)
    );
    assert!(
        portfolio
            .pending_orders_with_tag(&Tag::new("other"))
            .is_none()
    );
}

#[test]
fn cex_account_portfolio_rejects_pending_orders_grouped_under_a_mismatched_tag() {
    let order = PendingOrder::new(limit_sell(Quantity::OfBase(1.0), 100.0), 0.0, 0.0).unwrap();

    let result = CexAccountPortfolio::new(
        [(AssetId::Bitcoin, 10.0)],
        [(Tag::new("other"), vec![order])],
    );

    assert!(result.is_err());
}

#[test]
fn portfolio_builder_rejects_invalid_asset_counts() {
    let result = PortfolioBuilder::new().asset_count(
        CexId::Binance,
        AccountId::new("account"),
        AssetId::Bitcoin,
        -1.0,
    );

    assert!(result.is_err());
}

#[test]
fn portfolio_builder_rejects_invalid_pending_orders() {
    let result = PortfolioBuilder::new().pending_order(
        CexId::Binance,
        AccountId::new("account"),
        limit_sell(Quantity::OfBase(1.0), 100.0),
        -1.0,
        0.0,
    );

    assert!(result.is_err());
}

#[test]
fn portfolio_new_accepts_nested_iterators() {
    let cex_id = CexId::Binance;
    let account_id = AccountId::new("account");
    let account = CexAccountPortfolio::new(
        [(AssetId::Bitcoin, 2.0)],
        std::iter::empty::<(Tag, Vec<crate::pending_order::PendingOrder>)>(),
    )
    .unwrap();

    let portfolio = Portfolio::new([(cex_id, [(account_id.clone(), account.clone())])]).unwrap();
    assert!(portfolio.has_cex_account(&cex_id, &account_id));
    assert_eq!(
        portfolio.asset_total_in_cex_account(&cex_id, &account_id, &AssetId::Bitcoin),
        2.0
    );

    let other_account_id = AccountId::new("other");
    let merged = Portfolio::new([
        (cex_id, [(account_id.clone(), account.clone())]),
        (cex_id, [(other_account_id.clone(), account.clone())]),
    ])
    .unwrap();
    assert!(merged.has_cex_account(&cex_id, &account_id));
    assert!(merged.has_cex_account(&cex_id, &other_account_id));

    let flattened = Portfolio::new([(cex_id, account_id.clone(), account.clone())]).unwrap();
    assert_eq!(
        flattened.asset_total_in_cex_account(&cex_id, &account_id, &AssetId::Bitcoin),
        2.0
    );
}

#[test]
fn portfolio_new_rejects_duplicate_accounts() {
    let cex_id = CexId::Binance;
    let account_id = AccountId::new("account");
    let account = CexAccountPortfolio::new(
        [(AssetId::Bitcoin, 2.0)],
        std::iter::empty::<(Tag, Vec<crate::pending_order::PendingOrder>)>(),
    )
    .unwrap();

    let nested = Portfolio::new([
        (cex_id, [(account_id.clone(), account.clone())]),
        (cex_id, [(account_id.clone(), account.clone())]),
    ]);
    assert!(nested.is_err());

    let flattened = Portfolio::new([
        (cex_id, account_id.clone(), account.clone()),
        (cex_id, account_id, account),
    ]);
    assert!(flattened.is_err());
}
