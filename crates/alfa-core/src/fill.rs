/*
 * SPDX-FileCopyrightText: © 2022 The Alfa Authors <https://github.com/kyper-io/alfa/blob/913e3afe4177a9846b55c7c28c3f43a304736656/AUTHORS>
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use crate::{Cash, InstrumentId, NonZeroQuantity, Price, Quantity};

pub trait Fill {
    fn instrument_id(&self) -> &InstrumentId;
    fn quantity(&self) -> Quantity;
    fn price(&self) -> Price; // TODO: include time
    fn fee(&self) -> Cash;
}

// TODO: rename to Sales?
#[derive(Clone, Copy)]
pub struct FillLevel {
    pub price: Price,
    pub quantity: NonZeroQuantity,
}

pub struct SimulatorFill {
    pub instrument_id: InstrumentId,
    pub level: Option<FillLevel>,
    pub fee: Cash,
}

impl Fill for SimulatorFill {
    fn instrument_id(&self) -> &InstrumentId {
        &self.instrument_id
    }

    fn quantity(&self) -> Quantity {
        match self.level {
            Some(level) => level.quantity.get(),
            None => 0.0,
        }
    }

    fn price(&self) -> Price {
        self.level
            .expect("no price associated with zero quantity (simulator) fill")
            .price
    }

    fn fee(&self) -> Cash {
        self.fee
    }
}
