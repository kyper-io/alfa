/*
 * SPDX-FileCopyrightText: © 2022 The Alfa Authors <https://github.com/kyper-io/alfa/blob/913e3afe4177a9846b55c7c28c3f43a304736656/AUTHORS>
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

// TODO: refactor (into struct)
// TODO: attach context to errors?

// TODO: deny warnings in ci.yaml
// TODO: selectively warn from clippy::restriction?
#![warn(clippy::pedantic)]

mod backtest;
mod io;
mod order;

use backtest::backtest;
use io::{
    load_accounts, load_best_prices, load_signals, load_timestamps, load_universe,
    save_equity_curve,
};

use std::env;

fn main() -> anyhow::Result<()> {
    let dir = env::current_dir()?;

    let timestamps = load_timestamps(&dir)?;
    let universe = load_universe(&dir)?;
    let signals = load_signals(&dir, universe.len())?;
    let best_prices = load_best_prices(&dir, universe.len())?;
    let mut accounts = load_accounts(&dir)?;

    let equity_curve = backtest(
        &timestamps,
        &signals,
        &best_prices,
        &universe,
        accounts.pop().expect("no account configured"),
    )?;
    save_equity_curve(&dir, &equity_curve)?;

    Ok(())
}
