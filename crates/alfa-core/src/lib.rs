/*
 * SPDX-FileCopyrightText: © 2022 The Alfa Authors <https://github.com/kyper-io/alfa/blob/913e3afe4177a9846b55c7c28c3f43a304736656/AUTHORS>
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

// TODO: migrate to fixed-point arithmetic
// TODO: add more error checks
// TODO: add tracing
// TODO: mark as const where possible

// TODO: deny warnings in ci.yaml
// TODO: selectively warn from clippy::restriction?
#![warn(clippy::cargo, clippy::pedantic)]

mod account;
mod cash;
mod fee;
mod fill;
mod fill_model;
mod instrument;
mod order;
mod position;
mod price;
mod signal;
mod size;
mod trading_book;
mod util;

pub use account::{AccountConfig, StaticAccount};
pub use cash::Cash;
pub use fee::{Commission, MakerTaker};
pub use fill::{Fill, FillLevel, SimulatorFill};
pub use fill_model::{FillModel, TopOfBookFillModel, UpdateFillModel};
pub use instrument::{Instrument, InstrumentExt, InstrumentId, InstrumentSpec};
pub use order::Order;
pub use position::{FifoPosition, Position};
pub use price::{BestPrices, PositionPrices, Price};
pub use signal::{ExpiredSignal, Signal};
pub use size::{same_side, NonZeroQuantity, NotionalQuantity, Quantity};
pub use trading_book::StaticTradingBook;
pub use util::NotionalPercent;
