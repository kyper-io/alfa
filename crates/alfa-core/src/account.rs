/*
 * SPDX-FileCopyrightText: © 2022 The Alfa Authors <https://github.com/kyper-io/alfa/blob/913e3afe4177a9846b55c7c28c3f43a304736656/AUTHORS>
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use serde::{Deserialize, Serialize};

use crate::{Cash, FifoPosition, Fill, FillModel, Instrument, StaticTradingBook};

#[derive(Serialize, Deserialize)]
pub struct AccountConfig {
    name: String,
    initial_balance: Cash,
}

// TODO: remove bound on I?
pub struct StaticAccount<I: Instrument + Clone> {
    config: AccountConfig,
    balance: Cash,
    portfolio: StaticTradingBook<FifoPosition<I>>,
}

impl<I: Instrument + Clone> StaticAccount<I> {
    pub fn new(config: AccountConfig, universe: &[I]) -> Self {
        let balance = config.initial_balance;
        Self {
            config,
            balance,
            portfolio: StaticTradingBook::new(universe),
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.config.name
    }

    #[must_use]
    pub fn initial_balance(&self) -> Cash {
        self.config.initial_balance
    }

    #[must_use]
    pub fn balance(&self) -> Cash {
        self.balance
    }

    #[must_use]
    pub fn holdings(&self) -> &[FifoPosition<I>] {
        self.portfolio.holdings()
    }

    #[must_use]
    pub fn equity<F: FillModel>(&self, fill_models: &[F]) -> Cash {
        self.balance + self.portfolio.unrealized_pnl(fill_models)
    }

    pub fn reconcile<F: Fill>(&mut self, fills: &[F]) {
        let pnl = self.portfolio.reconcile(fills);
        self.balance += pnl;
    }
}
