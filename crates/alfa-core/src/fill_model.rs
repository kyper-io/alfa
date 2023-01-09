/*
 * SPDX-FileCopyrightText: © 2022 The Alfa Authors <https://github.com/kyper-io/alfa/blob/913e3afe4177a9846b55c7c28c3f43a304736656/AUTHORS>
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use std::convert::Infallible;

use crate::{
    BestPrices, Cash, Fill, FillLevel, Instrument, NonZeroQuantity, Order, Price, SimulatorFill,
};

pub trait FillModel {
    type Output: Fill;

    fn execute(&self, order: &Order) -> Self::Output;
}

pub trait UpdateFillModel<Args>: FillModel {
    type Error: std::error::Error;

    fn update(&mut self, args: Args) -> Result<(), Self::Error>;
}

// TODO: remove bound on I?
pub struct TopOfBookFillModel<I: Instrument> {
    instrument: I,
    prices: Option<BestPrices>,
}

impl<I: Instrument> TopOfBookFillModel<I> {
    pub fn new(instrument: I) -> Self {
        Self {
            instrument,
            prices: None,
        }
    }

    fn create_fill(&self, level: Option<FillLevel>, fee: Cash) -> SimulatorFill {
        SimulatorFill {
            instrument_id: self.instrument.unique_id().clone(),
            level,
            fee,
        }
    }
}

fn top_of_book_fill_price(quantity: NonZeroQuantity, prices: BestPrices) -> Price {
    if quantity.get() > 0.0 {
        prices.ask
    } else {
        prices.bid
    }
}

impl<I: Instrument> FillModel for TopOfBookFillModel<I> {
    type Output = SimulatorFill;

    fn execute(&self, order: &Order) -> Self::Output {
        match order {
            Order::Market {
                instrument_id,
                quantity,
            } => {
                assert_eq!(instrument_id, self.instrument.unique_id());

                match NonZeroQuantity::new(*quantity) {
                    Some(quantity) => {
                        let prices = self
                            .prices
                            .expect("'execute' should only be called after 'update'");
                        let price = top_of_book_fill_price(quantity, prices);
                        let fee = self.instrument.commission(quantity.get(), prices).taker;

                        self.create_fill(Some(FillLevel { price, quantity }), fee)
                    }
                    None => self.create_fill(None, 0.0),
                }
            }
        }
    }
}

impl<I: Instrument> UpdateFillModel<BestPrices> for TopOfBookFillModel<I> {
    type Error = Infallible; // TODO: replace with ! once stabilized

    fn update(&mut self, prices: BestPrices) -> Result<(), Self::Error> {
        self.prices = Some(prices);
        Ok(())
    }
}
