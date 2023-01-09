/*
 * SPDX-FileCopyrightText: © 2022 The Alfa Authors <https://github.com/kyper-io/alfa/blob/913e3afe4177a9846b55c7c28c3f43a304736656/AUTHORS>
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use serde::{Deserialize, Serialize};

use crate::{BestPrices, Cash, Commission, MakerTaker, NotionalQuantity, PositionPrices, Quantity};

// TODO: use static strings?
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct InstrumentId {
    venue: String,
    symbol: String,
}

pub trait Instrument {
    fn unique_id(&self) -> &InstrumentId;
    fn gross_pnl(&self, quantity: Quantity, prices: PositionPrices) -> Cash;
    fn commission(&self, quantity: Quantity, prices: BestPrices) -> MakerTaker<Cash>;
    fn to_transactable(&self, quantity: Quantity) -> Quantity; // TODO: return TransactableQuantity?
    fn to_notional(&self, quantity: Quantity) -> NotionalQuantity;
    fn to_settlement_value(quantity: NotionalQuantity, prices: BestPrices) -> Cash;
}

pub trait InstrumentExt: Instrument {
    fn venue(&self) -> &str;
    fn symbol(&self) -> &str;
    fn net_pnl(&self, quantity: Quantity, prices: PositionPrices, fee: Cash) -> Cash;
    fn to_notional_value(&self, quantity: Quantity, prices: BestPrices) -> Cash;
}

// Blanket implementation (rather than default ones) precludes user implementation
impl<I: Instrument> InstrumentExt for I {
    fn venue(&self) -> &str {
        &self.unique_id().venue
    }

    fn symbol(&self) -> &str {
        &self.unique_id().symbol
    }

    fn net_pnl(&self, quantity: Quantity, prices: PositionPrices, fee: Cash) -> Cash {
        self.gross_pnl(quantity, prices) - fee
    }

    fn to_notional_value(&self, quantity: Quantity, prices: BestPrices) -> Cash {
        Self::to_settlement_value(self.to_notional(quantity), prices)
    }
}

// TODO: support inverse payoffs
#[derive(Clone, Serialize, Deserialize)]
pub struct InstrumentSpec {
    unique_id: InstrumentId,
    multiplier: f64,
    commission: Commission,
}

impl Instrument for InstrumentSpec {
    fn unique_id(&self) -> &InstrumentId {
        &self.unique_id
    }

    fn gross_pnl(&self, quantity: Quantity, prices: PositionPrices) -> Cash {
        self.to_notional(quantity) * (prices.exit - prices.entry)
    }

    fn commission(&self, quantity: Quantity, prices: BestPrices) -> MakerTaker<Cash> {
        if quantity == 0.0 {
            MakerTaker::from_single(0.0)
        } else {
            match self.commission {
                Commission::Fixed(cash) => MakerTaker::from_single(cash),
                Commission::FixedPerUnit(cash) => MakerTaker::from_single(cash * quantity.abs()),
                Commission::FixedMakerTaker { maker, taker } => MakerTaker { maker, taker }
                    .map(|pct| pct.multiplier * self.to_notional_value(quantity.abs(), prices)),
            }
        }
    }

    fn to_transactable(&self, quantity: Quantity) -> Quantity {
        quantity.round() // TODO!
    }

    fn to_notional(&self, quantity: Quantity) -> NotionalQuantity {
        quantity * self.multiplier
    }

    fn to_settlement_value(quantity: NotionalQuantity, prices: BestPrices) -> Cash {
        quantity * prices.bid
    }
}
