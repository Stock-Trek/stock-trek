use super::pending_order::*;

use crate::error::result::StockTrekResult;
use stock_trek_types::cex::quantity::Quantity;

use stock_trek_types::cex::{
    activation::Activation, asset_id::AssetId, order_request::OrderRequest,
    price_basis::PriceBasis, side::Side, tag::Tag, time_in_force::TimeInForce,
    trigger_direction::TriggerDirection, trigger_mode::TriggerMode,
};

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

fn order_request() -> OrderRequest<AssetId, f64> {
    limit_sell(Quantity::OfBase(1.0), 100.0)
}

#[test]
fn accepts_non_negative_filled_quantities() {
    let order = PendingOrder::new(order_request(), 0.5, 50.0).unwrap();

    assert_eq!(order.filled_base_quantity(), 0.5);
    assert_eq!(order.filled_quote_quantity(), 50.0);
}

#[test]
fn rejects_negative_filled_quantities() {
    assert!(PendingOrder::new(order_request(), -0.1, 0.0).is_err());
    assert!(PendingOrder::new(order_request(), 0.0, -0.1).is_err());
}

#[test]
fn rejects_non_finite_filled_quantities() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(PendingOrder::new(order_request(), value, 0.0).is_err());
        assert!(PendingOrder::new(order_request(), 0.0, value).is_err());
    }
}

#[test]
fn rejects_non_positive_limit_prices() {
    for value in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(PendingOrder::new(limit_sell(Quantity::OfBase(1.0), value), 0.0, 0.0).is_err());
        assert!(PendingOrder::new(limit_sell(Quantity::OfQuote(100.0), value), 0.0, 0.0).is_err());
    }
}

#[test]
fn rejects_non_positive_limit_quantities() {
    for value in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(PendingOrder::new(limit_sell(Quantity::OfBase(value), 100.0), 0.0, 0.0).is_err());
        assert!(PendingOrder::new(limit_sell(Quantity::OfQuote(value), 100.0), 0.0, 0.0).is_err());
    }
}

#[test]
fn rejects_non_positive_market_order_quantities() {
    for value in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let market_buy = OrderRequest::MarketBuy {
            base: AssetId::Bitcoin,
            quote: AssetId::TetherUSD,
            activation: Activation::Immediate,
            quote_quantity: value,
            tag: Tag::new("tag"),
        };
        assert!(PendingOrder::new(market_buy, 0.0, 0.0).is_err());

        let market_sell = OrderRequest::MarketSell {
            base: AssetId::Bitcoin,
            quote: AssetId::TetherUSD,
            activation: Activation::Immediate,
            base_quantity: value,
            tag: Tag::new("tag"),
        };
        assert!(PendingOrder::new(market_sell, 0.0, 0.0).is_err());
    }
}

#[test]
fn rejects_non_finite_activation_prices() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(activation_price_triggered(value).is_err());
        assert!(trailing_activation_price(value).is_err());
    }
}

#[test]
fn rejects_non_positive_activation_prices() {
    for value in [0.0, -1.0] {
        assert!(activation_price_triggered(value).is_err());
        assert!(trailing_activation_price(value).is_err());
    }
}

#[test]
fn rejects_non_finite_reservation_amounts_derived_from_the_limit_price() {
    let limit_buy_with_overflowing_notional = OrderRequest::Limit {
        base: AssetId::Bitcoin,
        quote: AssetId::TetherUSD,
        side: Side::Buy,
        activation: Activation::Immediate,
        limit_price: f64::MAX,
        time_in_force: TimeInForce::GoodTillCancelled,
        quantity: Quantity::OfBase(f64::MAX),
        tag: Tag::new("tag"),
    };
    assert!(PendingOrder::new(limit_buy_with_overflowing_notional, 0.0, 0.0).is_err());

    let limit_sell_with_overflowing_base_quantity = OrderRequest::Limit {
        base: AssetId::Bitcoin,
        quote: AssetId::TetherUSD,
        side: Side::Sell,
        activation: Activation::Immediate,
        limit_price: 1.0e-300,
        time_in_force: TimeInForce::GoodTillCancelled,
        quantity: Quantity::OfQuote(f64::MAX),
        tag: Tag::new("tag"),
    };
    assert!(PendingOrder::new(limit_sell_with_overflowing_base_quantity, 0.0, 0.0).is_err());
}

fn activation_price_triggered(activation_price: f64) -> StockTrekResult<PendingOrder> {
    PendingOrder::new(
        OrderRequest::Limit {
            base: AssetId::Bitcoin,
            quote: AssetId::TetherUSD,
            side: Side::Sell,
            activation: Activation::PriceTriggered {
                activation_price,
                basis: PriceBasis::Last,
                direction: TriggerDirection::Above,
                mode: TriggerMode::Touch,
            },
            limit_price: 100.0,
            time_in_force: TimeInForce::GoodTillCancelled,
            quantity: Quantity::OfBase(1.0),
            tag: Tag::new("tag"),
        },
        0.0,
        0.0,
    )
}

fn trailing_activation_price(activation_price: f64) -> StockTrekResult<PendingOrder> {
    PendingOrder::new(
        OrderRequest::Limit {
            base: AssetId::Bitcoin,
            quote: AssetId::TetherUSD,
            side: Side::Sell,
            activation: Activation::Trailing {
                activation_price,
                basis: PriceBasis::Last,
                callback_rate_bps: 100,
                direction: TriggerDirection::Below,
            },
            limit_price: 100.0,
            time_in_force: TimeInForce::GoodTillCancelled,
            quantity: Quantity::OfBase(1.0),
            tag: Tag::new("tag"),
        },
        0.0,
        0.0,
    )
}
