use crate::{
    signal::key::SignalKey,
    value::{
        binary_operator::BinaryOperator,
        unary_operator::UnaryOperator,
        value::{AccountIdValue, AssetIdValue, CexIdValue, FlagValue, NumberValue},
    },
};
use stock_trek_types::cex::{account_id::AccountId, asset_id::AssetId, cex_id::CexId, tag::Tag};

/// Factory for values derived from the portfolio.
#[derive(Debug, Clone, Default)]
pub struct PortfolioValuesFactory;
/// Factory for calculated values.
#[derive(Debug, Clone, Default)]
pub struct CalculationValuesFactory;
/// Factory for literal values.
#[derive(Debug, Clone, Default)]
pub struct LiteralValuesFactory;
/// Factory for values read from signals.
#[derive(Debug, Clone, Default)]
pub struct SignalValuesFactory;

impl PortfolioValuesFactory {
    /// The total number of pending orders across all CEX accounts.
    pub fn pending_orders(&self) -> NumberValue {
        NumberValue::PendingOrders
    }
    /// The number of pending orders carrying `tag` across all CEX accounts.
    pub fn pending_orders_with_tag(&self, tag: Tag) -> NumberValue {
        NumberValue::PendingOrdersWithTag { tag }
    }
    /// The number of pending orders in a specific CEX account.
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
    /// The number of pending orders carrying `tag` in a specific CEX account.
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
    /// The total balance of an asset across all accounts, including reserved
    /// amounts.
    pub fn asset_total(&self, asset_id_value: AssetIdValue) -> NumberValue {
        NumberValue::AssetTotal { asset_id_value }
    }
    /// The total balance of an asset in a specific CEX account, including
    /// reserved amounts.
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
    /// The available (unreserved) balance of an asset across all accounts.
    pub fn asset_available(&self, asset_id_value: AssetIdValue) -> NumberValue {
        NumberValue::AssetAvailable { asset_id_value }
    }
    /// The available (unreserved) balance of an asset in a specific CEX
    /// account.
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
    /// The amount of an asset reserved by unfilled pending orders across all
    /// accounts.
    pub fn asset_reserved(&self, asset_id_value: AssetIdValue) -> NumberValue {
        NumberValue::AssetReserved { asset_id_value }
    }
    /// The amount of an asset reserved by unfilled pending orders in a specific
    /// CEX account.
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
    /// Applies a binary operator to two values when the calculation is
    /// resolved.
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
    /// Applies a unary operator to a value when the calculation is resolved.
    pub fn unary(&self, operator: UnaryOperator, number: NumberValue) -> NumberValue {
        NumberValue::UnaryCalculation {
            number: Box::new(number),
            operator,
        }
    }
}

impl LiteralValuesFactory {
    /// Wraps a concrete CEX identifier.
    pub fn cex_id(&self, literal: CexId) -> CexIdValue {
        CexIdValue::Literal { literal }
    }
    /// Wraps a concrete account identifier.
    pub fn account_id(&self, literal: AccountId) -> AccountIdValue {
        AccountIdValue::Literal { literal }
    }
    /// Wraps a concrete asset identifier.
    pub fn asset_id(&self, literal: AssetId) -> AssetIdValue {
        AssetIdValue::Literal { literal }
    }
    /// Wraps a concrete boolean flag.
    pub fn flag(&self, literal: bool) -> FlagValue {
        FlagValue::Literal { literal }
    }
    /// Wraps a concrete number.
    pub fn number(&self, literal: f64) -> NumberValue {
        NumberValue::Literal { literal }
    }
}

impl SignalValuesFactory {
    /// Reads a CEX identifier from a signal when the value is resolved.
    pub fn cex_id(&self, key: &SignalKey<CexId>) -> CexIdValue {
        CexIdValue::Signal {
            signal: key.clone(),
        }
    }
    /// Reads an account identifier from a signal when the value is resolved.
    pub fn account_id(&self, key: &SignalKey<AccountId>) -> AccountIdValue {
        AccountIdValue::Signal {
            signal: key.clone(),
        }
    }
    /// Reads an asset identifier from a signal when the value is resolved.
    pub fn asset_id(&self, key: &SignalKey<AssetId>) -> AssetIdValue {
        AssetIdValue::Signal {
            signal: key.clone(),
        }
    }
    /// Reads a boolean flag from a signal when the value is resolved.
    pub fn flag(&self, key: &SignalKey<bool>) -> FlagValue {
        FlagValue::Signal {
            signal: key.clone(),
        }
    }
    /// Reads a number from a signal when the value is resolved.
    pub fn number(&self, key: &SignalKey<f64>) -> NumberValue {
        NumberValue::Signal {
            signal: key.clone(),
        }
    }
}
