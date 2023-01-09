/*
 * SPDX-FileCopyrightText: © 2022 The Alfa Authors <https://github.com/kyper-io/alfa/blob/913e3afe4177a9846b55c7c28c3f43a304736656/AUTHORS>
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use std::iter::once;

use alfa_core::{
    AccountConfig, BestPrices, Cash, FillModel, Instrument, Order, Signal, SimulatorFill,
    StaticAccount, TopOfBookFillModel, UpdateFillModel,
};
use chrono::prelude::*;
use itertools::izip;
use ndarray::{prelude::*, Data};

use crate::order::order_quantity;

fn create_order<I>(instrument: &I) -> Order
where
    I: Instrument,
{
    Order::Market {
        instrument_id: instrument.unique_id().clone(),
        quantity: 0.0,
    }
}

fn create_fill_model<I>(instrument: &I) -> TopOfBookFillModel<I>
where
    I: Instrument + Clone,
{
    TopOfBookFillModel::new(instrument.clone())
}

fn create_fill<I>(instrument: &I) -> SimulatorFill
where
    I: Instrument,
{
    SimulatorFill {
        instrument_id: instrument.unique_id().clone(),
        level: None,
        fee: 0.0,
    }
}

// TODO: support multiple accounts
// TODO: reduce monomorphization bloat?
pub fn backtest<S1, S2, S3, I>(
    timestamps: &ArrayBase<S1, Ix1>,
    signals: &ArrayBase<S2, Ix2>,
    best_prices: &ArrayBase<S3, Ix2>,
    universe: &[I],
    account: AccountConfig,
) -> anyhow::Result<Array1<Cash>>
where
    S1: Data<Elem = DateTime<Utc>>,
    S2: Data<Elem = Signal>,
    S3: Data<Elem = BestPrices>,
    I: Instrument + Clone,
{
    assert_eq!(timestamps.shape()[0], signals.shape()[0]);
    assert_eq!(signals.shape(), best_prices.shape());
    assert_eq!(universe.len(), best_prices.shape()[1]);
    // TODO: check that timestamps are sorted once is_sorted is in std

    let mut equity_curve = Array1::zeros(timestamps.shape()[0]);

    let mut orders: Vec<_> = universe.iter().map(create_order).collect();
    let mut fill_models: Vec<_> = universe.iter().map(create_fill_model).collect();
    let mut fills: Vec<_> = universe.iter().map(create_fill).collect();
    let mut account = StaticAccount::new(account, universe);

    // TODO: can ordered iteration be done more efficiently?
    for (timestamp, prev_signals, signals_, best_prices_, equity) in izip!(
        timestamps.iter(),
        once(Array::zeros(signals.shape()[1]).view())
            .chain(signals.slice(s![..-1, ..]).outer_iter()),
        signals.outer_iter(),
        best_prices.outer_iter(),
        equity_curve.iter_mut(),
    ) {
        for (prev_signal, signal, prices, position, order, fill_model, fill) in izip!(
            prev_signals.iter(),
            signals_.iter(),
            best_prices_.iter(),
            account.holdings().iter(),
            orders.iter_mut(),
            fill_models.iter_mut(),
            fills.iter_mut()
        ) {
            match order {
                Order::Market {
                    instrument_id: _,
                    quantity,
                } => {
                    *quantity = order_quantity(*signal, *prev_signal, position, *prices, &account);
                }
            };
            fill_model.update(*prices)?;
            *fill = fill_model.execute(order);
        }
        account.reconcile(&fills);
        *equity = account.equity(&fill_models);
    }

    Ok(equity_curve)
}
