/*
 * SPDX-FileCopyrightText: © 2022 The Alfa Authors <https://github.com/kyper-io/alfa/blob/913e3afe4177a9846b55c7c28c3f43a304736656/AUTHORS>
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use itertools::Itertools;

use crate::{Cash, Fill, FillModel, Position};

pub struct StaticTradingBook<P: Position> {
    positions: Vec<P>,
}

impl<P: Position> StaticTradingBook<P> {
    pub fn new(universe: &[P::Underlying]) -> Self {
        Self {
            positions: universe
                .iter()
                .map(|underlying| P::new(underlying))
                .collect(),
        }
    }

    #[must_use]
    pub fn reconcile<F: Fill>(&mut self, fills: &[F]) -> Cash {
        self.positions
            .iter_mut()
            .zip_eq(fills)
            .map(|(position, fill)| position.reconcile(fill))
            .sum()
    }

    #[must_use]
    pub fn unrealized_pnl<F: FillModel>(&self, fill_models: &[F]) -> Cash {
        self.positions
            .iter()
            .zip_eq(fill_models)
            .map(|(position, fill_model)| position.unrealized_pnl(fill_model))
            .sum()
    }

    #[must_use]
    pub fn holdings(&self) -> &[P] {
        &self.positions
    }
}
