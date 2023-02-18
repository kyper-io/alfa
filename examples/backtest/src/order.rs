/*
 * SPDX-FileCopyrightText: © 2022 The Alfa Authors <https://github.com/kyper-io/alfa/blob/913e3afe4177a9846b55c7c28c3f43a304736656/AUTHORS>
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use alfa_core::{
    BestPrices, ExpiredSignal, Instrument, InstrumentExt, Position, Quantity, Signal, StaticAccount,
};

fn target_quantity<P, I>(
    signal: Signal,
    position: &P,
    prices: BestPrices,
    account: &StaticAccount<I>,
) -> Quantity
where
    P: Position,
    I: Instrument + Clone,
{
    // TODO: support generic compounding policies
    let capital = account.initial_balance();
    let unit_value = position.underlying().to_notional_value(1.0, prices);
    (signal * capital) / unit_value
}

fn target_quantity_unit(signal: Signal) -> Quantity {
    if signal > Signal::EPSILON {
        1.0
    } else if signal < -Signal::EPSILON {
        -1.0
    } else {
        0.0
    }
}

pub fn order_quantity<P, I>(
    signal: Signal,
    prev_signal: ExpiredSignal,
    position: &P,
    prices: BestPrices,
    account: &StaticAccount<I>,
) -> Quantity
where
    P: Position,
    I: Instrument + Clone,
{
    if signal == prev_signal {
        0.0
    } else {
        let delta_quantity = target_quantity_unit(signal) - position.net_quantity();
        position.underlying().to_transactable(delta_quantity)
    }
}
