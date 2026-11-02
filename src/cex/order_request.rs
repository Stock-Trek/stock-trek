use crate::{
    cex::{capability::HasRequiredCapabilities, cex_metadata::MarketMetadata},
    error::{numeric::NumericError, result::StockTrekResult},
    numeric::{Notional, Price, Quantity as ExactQuantity, decimal_from_calc, quantize},
    preferences::Preferences,
    resolvable::{Resolvable, ResolvableInCex},
    resolved_context::ResolvedContext,
    value::value::{AssetIdValue, NumberValue},
};
use rust_decimal::Decimal;
use stock_trek_types::cex::{
    activation::Activation, asset_id::AssetId, capability::CexCapability, cex_id::CexId,
    order_request::OrderRequest, quantity::Quantity, trigger_direction::TriggerDirection,
};

impl ResolvableInCex<OrderRequest<AssetId, Decimal>> for OrderRequest<AssetIdValue, NumberValue> {
    fn try_resolve_in_cex(
        &self,
        c: &ResolvedContext,
        cex_id: &CexId,
    ) -> StockTrekResult<OrderRequest<AssetId, Decimal>> {
        match self {
            Self::Limit {
                base,
                quote,
                side,
                activation,
                limit_price,
                time_in_force,
                quantity,
                tag,
            } => {
                let base = base.asset_id(c)?;
                let quote = quote.asset_id(c)?;
                let market = c.market_metadata(cex_id, &base, &quote)?;
                let limit_price = resolve_price(c, limit_price, market, &c.preferences)?;
                let activation = resolve_activation(c, activation, market, &c.preferences)?;
                let quantity = resolve_quantity(c, quantity, market, &c.preferences)?;
                let notional = match quantity {
                    Quantity::OfBase(base_quantity) => {
                        checked_mul(base_quantity, limit_price.to_decimal(), "order notional")?
                    }
                    Quantity::OfQuote(quote_quantity) => quote_quantity,
                };
                validate_minimum_notional(market, notional)?;
                Ok(OrderRequest::Limit {
                    base,
                    quote,
                    side: *side,
                    activation,
                    limit_price: limit_price.to_decimal(),
                    time_in_force: *time_in_force,
                    quantity,
                    tag: tag.clone(),
                })
            }
            Self::MarketBuy {
                base,
                quote,
                activation,
                quote_quantity,
                tag,
            } => {
                let base = base.asset_id(c)?;
                let quote = quote.asset_id(c)?;
                let market = c.market_metadata(cex_id, &base, &quote)?;
                let activation = resolve_activation(c, activation, market, &c.preferences)?;
                let quote_quantity = resolve_quantity_value(
                    c,
                    quote_quantity,
                    market.quote_increment,
                    &c.preferences,
                )?;
                validate_minimum_notional(market, quote_quantity.to_decimal())?;
                Ok(OrderRequest::MarketBuy {
                    base,
                    quote,
                    activation,
                    quote_quantity: quote_quantity.to_decimal(),
                    tag: tag.clone(),
                })
            }
            Self::MarketSell {
                base,
                quote,
                activation,
                base_quantity,
                tag,
            } => {
                let base = base.asset_id(c)?;
                let quote = quote.asset_id(c)?;
                let market = c.market_metadata(cex_id, &base, &quote)?;
                let activation = resolve_activation(c, activation, market, &c.preferences)?;
                let base_quantity = resolve_quantity_value(
                    c,
                    base_quantity,
                    market.base_increment,
                    &c.preferences,
                )?;
                Ok(OrderRequest::MarketSell {
                    base,
                    quote,
                    activation,
                    base_quantity: base_quantity.to_decimal(),
                    tag: tag.clone(),
                })
            }
        }
    }
}

fn resolve_price(
    c: &ResolvedContext,
    value: &NumberValue,
    market: &MarketMetadata,
    preferences: &Preferences,
) -> StockTrekResult<Price> {
    Price::try_from_calc_quantized(
        value.number(c)?,
        market.quote_increment,
        preferences.cex.rounding.price,
    )
}

fn resolve_quantity(
    c: &ResolvedContext,
    quantity: &Quantity<NumberValue>,
    market: &MarketMetadata,
    preferences: &Preferences,
) -> StockTrekResult<Quantity<Decimal>> {
    let quantity = quantity.try_resolve(c)?;
    match quantity {
        Quantity::OfBase(value) => Ok(Quantity::OfBase(
            quantize_quantity(value, market.base_increment, preferences)?.to_decimal(),
        )),
        Quantity::OfQuote(value) => Ok(Quantity::OfQuote(
            quantize_quantity(value, market.quote_increment, preferences)?.to_decimal(),
        )),
    }
}

fn resolve_quantity_value(
    c: &ResolvedContext,
    value: &NumberValue,
    increment: f64,
    preferences: &Preferences,
) -> StockTrekResult<ExactQuantity> {
    ExactQuantity::try_from_calc_quantized(
        value.number(c)?,
        increment,
        preferences.cex.rounding.quantity,
    )
}

fn quantize_quantity(
    value: Decimal,
    increment: f64,
    preferences: &Preferences,
) -> StockTrekResult<ExactQuantity> {
    let increment = decimal_from_calc("quantity increment", increment)?;
    let quantized = quantize(value, increment, preferences.cex.rounding.quantity)?;
    ExactQuantity::try_from_decimal(quantized)
}

fn resolve_activation(
    c: &ResolvedContext,
    activation: &Activation<NumberValue>,
    market: &MarketMetadata,
    preferences: &Preferences,
) -> StockTrekResult<Activation<Decimal>> {
    let activation = activation.try_resolve(c)?;
    match activation {
        Activation::Immediate => Ok(Activation::Immediate),
        Activation::PriceTriggered {
            activation_price,
            basis,
            direction,
            mode,
        } => {
            let activation_price =
                quantize_activation_price(activation_price, direction, market, preferences)?;
            Ok(Activation::PriceTriggered {
                activation_price,
                basis,
                direction,
                mode,
            })
        }
        Activation::Trailing {
            activation_price,
            basis,
            callback_rate_bps,
            direction,
        } => {
            let activation_price =
                quantize_activation_price(activation_price, direction, market, preferences)?;
            Ok(Activation::Trailing {
                activation_price,
                basis,
                callback_rate_bps,
                direction,
            })
        }
    }
}

fn quantize_activation_price(
    activation_price: Decimal,
    direction: TriggerDirection,
    market: &MarketMetadata,
    preferences: &Preferences,
) -> StockTrekResult<Decimal> {
    let rounding = match direction {
        TriggerDirection::Above => preferences.cex.rounding.activation_price_triggered_above,
        TriggerDirection::Below => preferences.cex.rounding.activation_price_triggered_below,
    };
    let increment = decimal_from_calc("quote increment", market.quote_increment)?;
    let quantized = quantize(activation_price, increment, rounding)?;
    Ok(Price::try_from_decimal(quantized)?.to_decimal())
}

fn validate_minimum_notional(market: &MarketMetadata, actual: Decimal) -> StockTrekResult<()> {
    if market.minimum_notional <= 0.0 {
        return Ok(());
    }
    let minimum = Notional::try_from_calc(market.minimum_notional)?;
    let actual = Notional::try_from_decimal(actual)?;
    if actual < minimum {
        return Err(NumericError::BelowMinimumNotional {
            minimum: minimum.to_decimal(),
            actual: actual.to_decimal(),
        }
        .into());
    }
    Ok(())
}

fn checked_mul(left: Decimal, right: Decimal, operation: &'static str) -> StockTrekResult<Decimal> {
    left.checked_mul(right)
        .ok_or_else(|| NumericError::Overflow { operation }.into())
}

impl<A, N> HasRequiredCapabilities for OrderRequest<A, N> {
    fn required_capabilities(&self) -> Vec<CexCapability> {
        match self {
            Self::Limit {
                quantity: Quantity::OfQuote(..),
                ..
            } => {
                vec![CexCapability::QuoteQuantityOnLimitOrders]
            }
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cex::cex_metadata::{CexMetadataBuilder, MarketMetadata},
        portfolio::Portfolio,
        preferences::Preferences,
        signal::signals::Signals,
    };
    use rust_decimal::RoundingStrategy;
    use std::collections::HashMap;
    use stock_trek_types::cex::{
        asset_id::AssetId,
        preferences::{CexPreferences, CexRoundingPreferences},
        side::Side,
        tag::Tag,
        time_in_force::TimeInForce,
    };

    fn preferences(
        activation_above: RoundingStrategy,
        activation_below: RoundingStrategy,
        price: RoundingStrategy,
        quantity: RoundingStrategy,
    ) -> Preferences {
        Preferences {
            cex: CexPreferences {
                max_network_delay_millis: 0,
                rounding: CexRoundingPreferences {
                    activation_price_triggered_above: activation_above,
                    activation_price_triggered_below: activation_below,
                    price,
                    quantity,
                    callback_rate_bps: RoundingStrategy::ToZero,
                },
            },
        }
    }

    fn context(market: MarketMetadata, preferences: Preferences) -> ResolvedContext {
        let mut metadata = CexMetadataBuilder::new();
        metadata.cex_market(CexId::Binance, AssetId::Bitcoin, AssetId::TetherUSD, market);
        ResolvedContext::new(
            Box::new(|_, _| Ok(())),
            Portfolio::new(HashMap::new()),
            Signals::new(),
            preferences,
            metadata.build(),
        )
    }

    fn market(minimum_notional: f64) -> MarketMetadata {
        MarketMetadata::new(0.001, 0.01, minimum_notional)
    }

    fn literal(value: f64) -> NumberValue {
        NumberValue::Literal { literal: value }
    }

    fn limit(
        limit_price: NumberValue,
        activation: Activation<NumberValue>,
        quantity: Quantity<NumberValue>,
    ) -> OrderRequest<AssetIdValue, NumberValue> {
        OrderRequest::Limit {
            base: AssetIdValue::Literal {
                literal: AssetId::Bitcoin,
            },
            quote: AssetIdValue::Literal {
                literal: AssetId::TetherUSD,
            },
            side: Side::Buy,
            activation,
            limit_price,
            time_in_force: TimeInForce::GoodTillCancelled,
            quantity,
            tag: Tag::new("test"),
        }
    }

    fn decimal(value: &str) -> Decimal {
        Decimal::from_str_exact(value).unwrap()
    }

    #[test]
    fn resolves_limit_values_to_quantized_decimal() {
        let request = limit(
            literal(100.005),
            Activation::PriceTriggered {
                activation_price: literal(100.005),
                basis: stock_trek_types::cex::price_basis::PriceBasis::Last,
                direction: TriggerDirection::Above,
                mode: stock_trek_types::cex::trigger_mode::TriggerMode::Cross,
            },
            Quantity::OfBase(literal(0.123_45)),
        );
        let preferences = preferences(
            RoundingStrategy::AwayFromZero,
            RoundingStrategy::ToZero,
            RoundingStrategy::ToZero,
            RoundingStrategy::ToZero,
        );
        let resolved = request
            .try_resolve_in_cex(&context(market(10.0), preferences), &CexId::Binance)
            .unwrap();

        match resolved {
            OrderRequest::Limit {
                limit_price,
                activation,
                quantity,
                ..
            } => {
                assert_eq!(limit_price, decimal("100.00"));
                assert!(matches!(
                    quantity,
                    Quantity::OfBase(value) if value == decimal("0.123")
                ));
                assert!(matches!(
                    activation,
                    Activation::PriceTriggered {
                        activation_price,
                        basis: stock_trek_types::cex::price_basis::PriceBasis::Last,
                        direction: TriggerDirection::Above,
                        mode: stock_trek_types::cex::trigger_mode::TriggerMode::Cross,
                    } if activation_price == decimal("100.01")
                ));
            }
            _ => panic!("unexpected order request"),
        }
    }

    #[test]
    fn uses_direction_specific_activation_rounding() {
        let below = limit(
            literal(100.0),
            Activation::PriceTriggered {
                activation_price: literal(100.005),
                basis: stock_trek_types::cex::price_basis::PriceBasis::Last,
                direction: TriggerDirection::Below,
                mode: stock_trek_types::cex::trigger_mode::TriggerMode::Cross,
            },
            Quantity::OfBase(literal(1.0)),
        );
        let preferences = preferences(
            RoundingStrategy::AwayFromZero,
            RoundingStrategy::ToZero,
            RoundingStrategy::ToZero,
            RoundingStrategy::ToZero,
        );
        let resolved = below
            .try_resolve_in_cex(&context(market(10.0), preferences), &CexId::Binance)
            .unwrap();

        match resolved {
            OrderRequest::Limit {
                activation:
                    Activation::PriceTriggered {
                        activation_price, ..
                    },
                ..
            } => assert_eq!(activation_price, decimal("100.00")),
            _ => panic!("unexpected order request"),
        }
    }

    #[test]
    fn rejects_orders_below_the_minimum_notional() {
        let request = limit(
            literal(1.0),
            Activation::Immediate,
            Quantity::OfBase(literal(1.0)),
        );
        let preferences = preferences(
            RoundingStrategy::ToZero,
            RoundingStrategy::ToZero,
            RoundingStrategy::ToZero,
            RoundingStrategy::ToZero,
        );
        let result =
            request.try_resolve_in_cex(&context(market(10.0), preferences), &CexId::Binance);

        assert!(matches!(
            result,
            Err(crate::error::result::StockTrekError::Numeric(
                NumericError::BelowMinimumNotional { .. }
            ))
        ));
    }

    #[test]
    fn rejects_non_finite_order_values() {
        let request = limit(
            literal(f64::NAN),
            Activation::Immediate,
            Quantity::OfBase(literal(1.0)),
        );
        let preferences = preferences(
            RoundingStrategy::ToZero,
            RoundingStrategy::ToZero,
            RoundingStrategy::ToZero,
            RoundingStrategy::ToZero,
        );
        let result =
            request.try_resolve_in_cex(&context(market(0.0), preferences), &CexId::Binance);

        assert!(matches!(
            result,
            Err(crate::error::result::StockTrekError::Numeric(
                NumericError::NonFinite { .. }
            ))
        ));
    }
}
