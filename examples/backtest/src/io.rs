/*
 * SPDX-FileCopyrightText: © 2022 The Alfa Authors <https://github.com/kyper-io/alfa/blob/913e3afe4177a9846b55c7c28c3f43a304736656/AUTHORS>
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

// TODO: delete this file

use std::{fs::File, path::Path};

use alfa_core::{AccountConfig, BestPrices, Cash, InstrumentSpec, Signal};
use chrono::prelude::*;
use ndarray::{prelude::*, Data};
use polars::prelude::*;

pub fn load_timestamps(dir: &Path) -> anyhow::Result<Array1<DateTime<Utc>>> {
    let timestamps = CsvReader::from_path(dir.join("timestamps.csv"))?
        .with_dtypes_slice(Some(&[DataType::Int64]))
        .has_header(false)
        .with_parse_dates(false)
        .finish()?
        .to_ndarray::<Int64Type>()?;
    assert_eq!(timestamps.ncols(), 1);
    let timestamps = timestamps
        .column(0)
        .mapv(|ts| Utc.timestamp_opt(ts, 0).unwrap());
    Ok(timestamps)
}

pub fn load_signals(dir: &Path, ncols: usize) -> anyhow::Result<Array2<Signal>> {
    let signals = CsvReader::from_path(dir.join("signals.csv"))?
        .with_dtypes_slice(Some(&vec![DataType::Float64; ncols]))
        .has_header(false)
        .with_parse_dates(false)
        .finish()?
        .to_ndarray::<Float64Type>()?;
    assert_eq!(signals.ncols(), ncols);
    Ok(signals)
}

pub fn load_best_prices(dir: &Path, ncols: usize) -> anyhow::Result<Array2<BestPrices>> {
    let prices = CsvReader::from_path(dir.join("prices.csv"))?
        .with_dtypes_slice(Some(&vec![DataType::Float64; ncols]))
        .has_header(false)
        .with_parse_dates(false)
        .finish()?
        .to_ndarray::<Float64Type>()?;
    assert_eq!(prices.ncols(), ncols);
    let prices = prices.mapv(BestPrices::from_single);
    Ok(prices)
}

pub fn load_universe(dir: &Path) -> anyhow::Result<Vec<InstrumentSpec>> {
    let file = File::open(dir.join("universe.json"))?;
    let universe = serde_json::from_reader(file)?;
    Ok(universe)
}

pub fn load_accounts(dir: &Path) -> anyhow::Result<Vec<AccountConfig>> {
    let file = File::open(dir.join("accounts.json"))?;
    let accounts = serde_json::from_reader(file)?;
    Ok(accounts)
}

pub fn save_equity_curve<S>(dir: &Path, equity_curve: &ArrayBase<S, Ix1>) -> anyhow::Result<()>
where
    S: Data<Elem = Cash>,
{
    let mut equity_curve = DataFrame::new(vec![Series::from_iter(equity_curve)])?;
    let mut file = File::create(dir.join("equity_curve.csv"))?;
    CsvWriter::new(&mut file)
        .has_header(false)
        .finish(&mut equity_curve)?;
    Ok(())
}
