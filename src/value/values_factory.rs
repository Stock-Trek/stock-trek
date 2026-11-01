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
#[derive(Debug, Clone)]
pub struct PortfolioValuesFactory;
/// Factory for calculated values.
#[derive(Debug, Clone)]
pub struct CalculationValuesFactory;
/// Factory for literal values.
#[derive(Debug, Clone)]
pub struct LiteralValuesFactory;
/// Factory for values read from signals.
#[derive(Debug, Clone)]
pub struct SignalValuesFactory;

impl PortfolioValuesFactory {
    /// Creates a value for the total number of pending orders.
    pub fn pending_orders(&self) -> NumberValue {
        NumberValue::PendingOrders
    }
    /// Creates a value for the number of pending orders with a tag.
    pub fn pending_orders_with_tag(&self, tag: Tag) -> NumberValue {
        NumberValue::PendingOrdersWithTag { tag }
    }
    /// Creates a value for the number of pending orders in a CEX account.
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
    /// Creates a value for the number of pending orders with a tag in a CEX account.
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
    /// Creates a value for the total amount of an asset.
    pub fn asset_total(&self, asset_id_value: AssetIdValue) -> NumberValue {
        NumberValue::AssetTotal { asset_id_value }
    }
    /// Creates a value for the total amount of an asset in a CEX account.
    pub fn asset_total_in_cex_account(
        &self,
        asset_id_value: AssetIdValue,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
    ) -> NumberValue {
        NumberValue::AssetTotalInCexAccount {
            asset_id_value,
            cex_id_value,
            account_id_value,
        }
    }
    /// Creates a value for the available amount of an asset.
    pub fn asset_available(&self, asset_id_value: AssetIdValue) -> NumberValue {
        NumberValue::AssetAvailable { asset_id_value }
    }
    /// Creates a value for the available amount of an asset in a CEX account.
    pub fn asset_available_in_cex_account(
        &self,
        asset_id_value: AssetIdValue,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
    ) -> NumberValue {
        NumberValue::AssetAvailableInCexAccount {
            asset_id_value,
            cex_id_value,
            account_id_value,
        }
    }
    /// Creates a value for the reserved amount of an asset.
    pub fn asset_reserved(&self, asset_id_value: AssetIdValue) -> NumberValue {
        NumberValue::AssetReserved { asset_id_value }
    }
    /// Creates a value for the reserved amount of an asset in a CEX account.
    pub fn asset_reserved_in_cex_account(
        &self,
        asset_id_value: AssetIdValue,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
    ) -> NumberValue {
        NumberValue::AssetReservedInCexAccount {
            asset_id_value,
            cex_id_value,
            account_id_value,
        }
    }
}

impl CalculationValuesFactory {
    /// Creates a binary calculation value.
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
    /// Creates a unary calculation value.
    pub fn unary(&self, operator: UnaryOperator, number: NumberValue) -> NumberValue {
        NumberValue::UnaryCalculation {
            number: Box::new(number),
            operator,
        }
    }
}

impl LiteralValuesFactory {
    /// Creates a literal CEX identifier.
    pub fn cex_id(&self, literal: CexId) -> CexIdValue {
        CexIdValue::Literal { literal }
    }
    /// Creates a literal account identifier.
    pub fn account_id(&self, literal: AccountId) -> AccountIdValue {
        AccountIdValue::Literal { literal }
    }
    /// Creates a literal asset identifier.
    pub fn asset_id(&self, literal: AssetId) -> AssetIdValue {
        AssetIdValue::Literal { literal }
    }
    /// Creates a literal boolean flag.
    pub fn flag(&self, literal: bool) -> FlagValue {
        FlagValue::Literal { literal }
    }
    /// Creates a literal number.
    pub fn number(&self, literal: f64) -> NumberValue {
        NumberValue::Literal { literal }
    }
}

impl SignalValuesFactory {
    /// Creates a CEX identifier value read from a signal.
    pub fn cex_id(&self, key: &SignalKey<CexId>) -> CexIdValue {
        CexIdValue::Signal {
            signal: key.clone(),
        }
    }
    /// Creates an account identifier value read from a signal.
    pub fn account_id(&self, key: &SignalKey<AccountId>) -> AccountIdValue {
        AccountIdValue::Signal {
            signal: key.clone(),
        }
    }
    /// Creates an asset identifier value read from a signal.
    pub fn asset_id(&self, key: &SignalKey<AssetId>) -> AssetIdValue {
        AssetIdValue::Signal {
            signal: key.clone(),
        }
    }
    /// Creates a boolean flag value read from a signal.
    pub fn flag(&self, key: &SignalKey<bool>) -> FlagValue {
        FlagValue::Signal {
            signal: key.clone(),
        }
    }
    /// Creates a numeric value read from a signal.
    pub fn number(&self, key: &SignalKey<f64>) -> NumberValue {
        NumberValue::Signal {
            signal: key.clone(),
        }
    }
}
