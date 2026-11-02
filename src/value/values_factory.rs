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

#[allow(missing_docs)]
impl PortfolioValuesFactory {
    pub fn pending_orders(&self) -> NumberValue {
        NumberValue::PendingOrders
    }
    pub fn pending_orders_with_tag(&self, tag: Tag) -> NumberValue {
        NumberValue::PendingOrdersWithTag { tag }
    }
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
    pub fn asset_total(&self, asset_id_value: AssetIdValue) -> NumberValue {
        NumberValue::AssetTotal { asset_id_value }
    }
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
    pub fn asset_available(&self, asset_id_value: AssetIdValue) -> NumberValue {
        NumberValue::AssetAvailable { asset_id_value }
    }
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
    pub fn asset_reserved(&self, asset_id_value: AssetIdValue) -> NumberValue {
        NumberValue::AssetReserved { asset_id_value }
    }
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

#[allow(missing_docs)]
impl CalculationValuesFactory {
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
    pub fn unary(&self, operator: UnaryOperator, number: NumberValue) -> NumberValue {
        NumberValue::UnaryCalculation {
            number: Box::new(number),
            operator,
        }
    }
}

#[allow(missing_docs)]
impl LiteralValuesFactory {
    pub fn cex_id(&self, literal: CexId) -> CexIdValue {
        CexIdValue::Literal { literal }
    }
    pub fn account_id(&self, literal: AccountId) -> AccountIdValue {
        AccountIdValue::Literal { literal }
    }
    pub fn asset_id(&self, literal: AssetId) -> AssetIdValue {
        AssetIdValue::Literal { literal }
    }
    pub fn flag(&self, literal: bool) -> FlagValue {
        FlagValue::Literal { literal }
    }
    pub fn number(&self, literal: f64) -> NumberValue {
        NumberValue::Literal { literal }
    }
}

#[allow(missing_docs)]
impl SignalValuesFactory {
    pub fn cex_id(&self, key: &SignalKey<CexId>) -> CexIdValue {
        CexIdValue::Signal {
            signal: key.clone(),
        }
    }
    pub fn account_id(&self, key: &SignalKey<AccountId>) -> AccountIdValue {
        AccountIdValue::Signal {
            signal: key.clone(),
        }
    }
    pub fn asset_id(&self, key: &SignalKey<AssetId>) -> AssetIdValue {
        AssetIdValue::Signal {
            signal: key.clone(),
        }
    }
    pub fn flag(&self, key: &SignalKey<bool>) -> FlagValue {
        FlagValue::Signal {
            signal: key.clone(),
        }
    }
    pub fn number(&self, key: &SignalKey<f64>) -> NumberValue {
        NumberValue::Signal {
            signal: key.clone(),
        }
    }
}
