/*
 * SPDX-FileCopyrightText: © 2022 The Alfa Authors <https://github.com/kyper-io/alfa/blob/913e3afe4177a9846b55c7c28c3f43a304736656/AUTHORS>
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use std::collections::VecDeque;

use itertools::Itertools;

use crate::{
    same_side, Cash, Fill, FillModel, Instrument, InstrumentExt, Order, PositionPrices, Price,
    Quantity,
};

pub trait Position {
    type Underlying: Instrument; // TODO: rename to avoid confusion with derivatives?

    fn new(underlying: &Self::Underlying) -> Self;
    fn underlying(&self) -> &Self::Underlying;
    fn reconcile<F: Fill>(&mut self, fill: &F) -> Cash;
    fn unrealized_pnl<F: FillModel>(&self, fill_model: &F) -> Cash;
    fn net_quantity(&self) -> Quantity;
}

#[derive(Clone, Copy)]
struct PositionLeg {
    quantity: Quantity, // TODO: maintain net quantity in Position once migrated to fixed-point arithmetic
    entry_price: Price, // TODO: include entry time
}

// TODO: replace with generic weighted mean extension method
fn average_entry_price<'a, L>(position_legs: L, net_quantity: Quantity) -> Price
where
    L: IntoIterator<Item = &'a PositionLeg>,
{
    position_legs
        .into_iter()
        .map(|leg| leg.quantity * leg.entry_price)
        .sum::<Price>()
        / net_quantity
}

// TODO: remove bound on I?
pub struct FifoPosition<I: Instrument> {
    underlying: I,
    legs: VecDeque<PositionLeg>,
}

impl<I: Instrument> FifoPosition<I> {
    fn new(underlying: I) -> Self {
        Self {
            underlying,
            legs: VecDeque::new(),
        }
    }

    fn gross_pnl_one_leg(&self, leg: PositionLeg, exit_price: Price) -> Cash {
        let prices = PositionPrices {
            entry: leg.entry_price,
            exit: exit_price,
        };
        self.underlying.gross_pnl(leg.quantity, prices)
    }

    fn gross_pnl_all_legs(&self, exit_price: Price) -> Cash {
        self.legs
            .iter()
            .map(|leg| self.gross_pnl_one_leg(*leg, exit_price))
            .sum()
    }

    fn accumulate_gross(&mut self, fill_quantity: Quantity, fill_price: Price) -> Cash {
        let leg = PositionLeg {
            quantity: fill_quantity,
            entry_price: fill_price,
        };
        self.legs.push_back(leg);
        0.0
    }

    fn close_gross(&mut self, fill_price: Price) -> Cash {
        let pnl = self.gross_pnl_all_legs(fill_price);
        self.legs.clear();
        pnl
    }

    fn close_first_leg_gross(&mut self, exit_price: Price) -> Cash {
        let pnl = self.gross_pnl_one_leg(
            *self.legs.front().expect("no first leg to close"),
            exit_price,
        );
        self.legs.pop_front();
        pnl
    }

    fn reduce_gross(&mut self, mut fill_quantity: Quantity, fill_price: Price) -> Cash {
        let mut pnl: Cash = 0.0;
        loop {
            let front_leg = self.legs.front_mut().expect("no front leg to reduce");
            let quantity = front_leg.quantity + fill_quantity;
            if quantity == 0.0 {
                pnl += self.close_first_leg_gross(fill_price);
                return pnl;
            } else if same_side(quantity, front_leg.quantity) {
                front_leg.quantity = quantity;
                let prices = PositionPrices {
                    entry: front_leg.entry_price,
                    exit: fill_price,
                };
                pnl += self.underlying.gross_pnl(-fill_quantity, prices);
                return pnl;
            }
            pnl += self.close_first_leg_gross(fill_price);
            fill_quantity = quantity;
        }
    }

    fn reverse_gross(&mut self, target_quantity: Quantity, fill_price: Price) -> Cash {
        let pnl = self.gross_pnl_all_legs(fill_price);
        self.legs.resize_with(1, || unreachable!());
        let front_leg = self
            .legs
            .front_mut()
            .expect("no reverse front leg to assign to");
        front_leg.quantity = target_quantity;
        front_leg.entry_price = fill_price;
        pnl
    }

    fn reconcile_gross<F: Fill>(&mut self, fill: &F, current_quantity: Quantity) -> Cash {
        const EPSILON: Quantity = 0.000_001 * 0.5; // TODO: deduce from minimum quantity

        assert!(self.all_legs_same_side());

        if current_quantity == 0.0 || same_side(fill.quantity(), current_quantity) {
            self.accumulate_gross(fill.quantity(), fill.price())
        } else {
            let target_quantity = current_quantity + fill.quantity();
            if target_quantity.abs() < EPSILON {
                self.close_gross(fill.price())
            } else if same_side(target_quantity, current_quantity) {
                self.reduce_gross(fill.quantity(), fill.price())
            } else {
                self.reverse_gross(target_quantity, fill.price())
            }
        }
    }

    fn unrealized_pnl<F: FillModel>(&self, fill_model: &F, current_quantity: Quantity) -> Cash {
        assert!(self.all_legs_same_side());

        let order = Order::Market {
            instrument_id: self.underlying.unique_id().clone(),
            quantity: -current_quantity,
        };
        let fill = fill_model.execute(&order);
        let prices = PositionPrices {
            entry: average_entry_price(&self.legs, current_quantity),
            exit: fill.price(),
        };
        self.underlying
            .net_pnl(current_quantity, prices, fill.fee())
    }

    fn all_legs_same_side(&self) -> bool {
        self.legs
            .iter()
            .map(|leg| leg.quantity.signum())
            .all_equal()
    }
}

impl<I: Instrument + Clone> Position for FifoPosition<I> {
    type Underlying = I;

    fn new(underlying: &Self::Underlying) -> Self {
        Self::new(underlying.clone())
    }

    fn underlying(&self) -> &Self::Underlying {
        &self.underlying
    }

    fn reconcile<F: Fill>(&mut self, fill: &F) -> Cash {
        assert_eq!(fill.instrument_id(), self.underlying.unique_id());

        if fill.quantity() == 0.0 {
            -fill.fee()
        } else {
            self.reconcile_gross(fill, self.net_quantity()) - fill.fee()
        }
    }

    // Assumes close at market rate
    fn unrealized_pnl<F: FillModel>(&self, fill_model: &F) -> Cash {
        if self.legs.is_empty() {
            0.0
        } else {
            self.unrealized_pnl(fill_model, self.net_quantity())
        }
    }

    fn net_quantity(&self) -> Quantity {
        if self.legs.is_empty() {
            0.0
        } else {
            self.legs.iter().map(|leg| leg.quantity).sum()
        }
    }
}
