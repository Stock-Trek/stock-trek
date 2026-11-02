use crate::{
    signal::key::SignalKey,
    value::{
        binary_operator::BinaryOperator,
        unary_operator::UnaryOperator,
        value_types::{AccountIdValue, AssetIdValue, CexIdValue, FlagValue, NumberValue},
    },
};
use stock_trek_types::cex::{account_id::AccountId, asset_id::AssetId, cex_id::CexId, tag::Tag};

/// Factory for values derived from the portfolio.
///
/// Available as the `portfolio` field of a [`StrategyContext`]. The values it
/// produces are re-read from the latest portfolio each time the command tree
/// is interpreted.
///
/// [`StrategyContext`]: crate::StrategyContext
#[derive(Debug, Clone, Default)]
pub struct PortfolioValuesFactory;
/// Factory for calculated values.
///
/// Available as the `calculations` field of a [`StrategyContext`].
///
/// [`StrategyContext`]: crate::StrategyContext
#[derive(Debug, Clone, Default)]
pub struct CalculationValuesFactory;
/// Factory for values fixed at strategy-build time.
///
/// Available as the `literals` field of a [`StrategyContext`].
///
/// [`StrategyContext`]: crate::StrategyContext
#[derive(Debug, Clone, Default)]
pub struct LiteralValuesFactory;
/// Factory for values read from the signals of the current evaluation.
///
/// Available as the `signals` field of a [`StrategyContext`].
///
/// [`StrategyContext`]: crate::StrategyContext
#[derive(Debug, Clone, Default)]
pub struct SignalValuesFactory;

impl PortfolioValuesFactory {
    /// The number of pending orders across all CEX accounts.
    pub fn pending_orders(&self) -> NumberValue {
        NumberValue::PendingOrders
    }
    /// The number of pending orders carrying the tag, across all CEX
    /// accounts.
    pub fn pending_orders_with_tag(&self, tag: Tag) -> NumberValue {
        NumberValue::PendingOrdersWithTag { tag }
    }
    /// The number of pending orders in a CEX account.
    pub fn pending_orders_in_cex_account(
        &self,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
    ) -> NumberValue {
        NumberValue::PendingOrdersInCexAccount {
            cex_id_value,
            account_id_value,
        }
    }
    /// The number of pending orders carrying the tag in a CEX account.
    pub fn pending_orders_in_cex_account_with_tag(
        &self,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
        tag: Tag,
    ) -> NumberValue {
        NumberValue::PendingOrdersInCexAccountWithTag {
            cex_id_value,
            account_id_value,
            tag,
        }
    }
    /// The amount of an asset held across all CEX accounts, available and
    /// reserved.
    pub fn asset_total(&self, asset_id_value: AssetIdValue) -> NumberValue {
        NumberValue::AssetTotal { asset_id_value }
    }
    /// The amount of an asset held in a CEX account, available and reserved.
    pub fn asset_total_in_cex_account(
        &self,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
        asset_id_value: AssetIdValue,
    ) -> NumberValue {
        NumberValue::AssetTotalInCexAccount {
            asset_id_value,
            cex_id_value,
            account_id_value,
        }
    }
    /// The amount of an asset across all CEX accounts that is not reserved
    /// for pending orders.
    pub fn asset_available(&self, asset_id_value: AssetIdValue) -> NumberValue {
        NumberValue::AssetAvailable { asset_id_value }
    }
    /// The amount of an asset in a CEX account that is not reserved for
    /// pending orders.
    pub fn asset_available_in_cex_account(
        &self,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
        asset_id_value: AssetIdValue,
    ) -> NumberValue {
        NumberValue::AssetAvailableInCexAccount {
            asset_id_value,
            cex_id_value,
            account_id_value,
        }
    }
    /// The amount of an asset across all CEX accounts that is reserved for
    /// pending orders.
    pub fn asset_reserved(&self, asset_id_value: AssetIdValue) -> NumberValue {
        NumberValue::AssetReserved { asset_id_value }
    }
    /// The amount of an asset in a CEX account that is reserved for pending
    /// orders.
    pub fn asset_reserved_in_cex_account(
        &self,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
        asset_id_value: AssetIdValue,
    ) -> NumberValue {
        NumberValue::AssetReservedInCexAccount {
            asset_id_value,
            cex_id_value,
            account_id_value,
        }
    }
}

impl CalculationValuesFactory {
    /// Applies a binary operator to two values.
    pub fn binary(
        &self,
        left: NumberValue,
        operator: BinaryOperator,
        right: NumberValue,
    ) -> NumberValue {
        NumberValue::BinaryCalculation {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
    /// Applies a unary operator to a value.
    pub fn unary(&self, operator: UnaryOperator, number: NumberValue) -> NumberValue {
        NumberValue::UnaryCalculation {
            number: Box::new(number),
            operator,
        }
    }
}

impl LiteralValuesFactory {
    /// A value that always evaluates to the CEX identifier.
    pub fn cex_id(&self, literal: CexId) -> CexIdValue {
        CexIdValue::Literal { literal }
    }
    /// A value that always evaluates to the account identifier.
    pub fn account_id(&self, literal: AccountId) -> AccountIdValue {
        AccountIdValue::Literal { literal }
    }
    /// A value that always evaluates to the asset identifier.
    pub fn asset_id(&self, literal: AssetId) -> AssetIdValue {
        AssetIdValue::Literal { literal }
    }
    /// A value that always evaluates to the flag.
    pub fn flag(&self, literal: bool) -> FlagValue {
        FlagValue::Literal { literal }
    }
    /// A value that always evaluates to the number.
    pub fn number(&self, literal: f64) -> NumberValue {
        NumberValue::Literal { literal }
    }
}

impl SignalValuesFactory {
    /// A value that evaluates to the CEX identifier signal.
    pub fn cex_id(&self, key: &SignalKey<CexId>) -> CexIdValue {
        CexIdValue::Signal {
            signal: key.clone(),
        }
    }
    /// A value that evaluates to the account identifier signal.
    pub fn account_id(&self, key: &SignalKey<AccountId>) -> AccountIdValue {
        AccountIdValue::Signal {
            signal: key.clone(),
        }
    }
    /// A value that evaluates to the asset identifier signal.
    pub fn asset_id(&self, key: &SignalKey<AssetId>) -> AssetIdValue {
        AssetIdValue::Signal {
            signal: key.clone(),
        }
    }
    /// A value that evaluates to the flag signal.
    pub fn flag(&self, key: &SignalKey<bool>) -> FlagValue {
        FlagValue::Signal {
            signal: key.clone(),
        }
    }
    /// A value that evaluates to the number signal.
    pub fn number(&self, key: &SignalKey<f64>) -> NumberValue {
        NumberValue::Signal {
            signal: key.clone(),
        }
    }
}
