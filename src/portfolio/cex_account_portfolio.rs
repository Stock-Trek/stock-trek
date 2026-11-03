use crate::{error::result::StockTrekResult, pending_order::PendingOrder, validation::Validation};
use std::collections::BTreeMap;
use stock_trek_types::cex::{asset_id::AssetId, tag::Tag};

/// Asset balances and pending orders for a single CEX account.
#[derive(Debug, Clone, Default)]
pub struct CexAccountPortfolio {
    pub(super) asset_counts: BTreeMap<AssetId, f64>,
    pub(super) pending_orders: BTreeMap<Tag, Vec<PendingOrder>>,
}

impl CexAccountPortfolio {
    /// Creates an account portfolio from asset balances and pending orders.
    ///
    /// Both arguments accept any iterator of key/value pairs. Returns an error
    /// when an asset balance appears more than once, when any asset balance is
    /// negative or not finite, when a pending-order tag appears more than once,
    /// or when a pending order's tag does not match the key of the group it is
    /// listed under.
    pub fn new<A, P>(asset_counts: A, pending_orders: P) -> StockTrekResult<Self>
    where
        A: IntoIterator<Item = (AssetId, f64)>,
        P: IntoIterator<Item = (Tag, Vec<PendingOrder>)>,
    {
        let mut balances = BTreeMap::new();
        for (asset_id, count) in asset_counts {
            Validation::non_negative("CexAccountPortfolio::new", "asset balance", count)?;
            if balances.insert(asset_id, count).is_some() {
                return Err(Validation::error(
                    "CexAccountPortfolio::new",
                    format!("duplicate asset balance for asset {asset_id}"),
                ));
            }
        }

        let mut orders_by_tag = BTreeMap::new();
        for (tag, orders) in pending_orders {
            if let Some(order) = orders.iter().find(|order| order.tag() != &tag) {
                return Err(Validation::error(
                    "CexAccountPortfolio::new",
                    format!(
                        "pending order tag {:?} does not match group tag {:?}",
                        order.tag(),
                        tag
                    ),
                ));
            }
            if orders_by_tag.insert(tag.clone(), orders).is_some() {
                return Err(Validation::error(
                    "CexAccountPortfolio::new",
                    format!("duplicate pending orders for tag {tag:?}"),
                ));
            }
        }

        Ok(Self {
            asset_counts: balances,
            pending_orders: orders_by_tag,
        })
    }

    /// Iterates over all asset balances keyed by asset.
    pub fn asset_counts(&self) -> impl Iterator<Item = (&AssetId, &f64)> {
        self.asset_counts.iter()
    }

    /// Returns the total balance (available and reserved) of an asset.
    ///
    /// Returns zero when the account does not hold the asset.
    pub fn asset_count(&self, asset_id: &AssetId) -> f64 {
        self.asset_counts.get(asset_id).copied().unwrap_or(0.0)
    }

    /// Iterates over all pending orders grouped by tag.
    pub fn pending_orders(&self) -> impl Iterator<Item = (&Tag, &[PendingOrder])> {
        self.pending_orders
            .iter()
            .map(|(tag, orders)| (tag, orders.as_slice()))
    }

    /// Returns the pending orders with the given tag, if any.
    pub fn pending_orders_with_tag(&self, tag: &Tag) -> Option<&[PendingOrder]> {
        self.pending_orders.get(tag).map(Vec::as_slice)
    }

    /// Iterates over every pending order together with its tag.
    pub fn pending_orders_iter(&self) -> impl Iterator<Item = (&Tag, &PendingOrder)> {
        self.pending_orders
            .iter()
            .flat_map(|(tag, orders)| orders.iter().map(move |order| (tag, order)))
    }
}
