/*
 * SPDX-FileCopyrightText: © 2022 The Alfa Authors <https://github.com/kyper-io/alfa/blob/913e3afe4177a9846b55c7c28c3f43a304736656/AUTHORS>
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

// TODO: strong typing
pub type Price = f64;

#[derive(Clone, Copy)]
pub struct BestPrices {
    pub ask: Price,
    pub bid: Price,
}

impl BestPrices {
    #[must_use]
    pub fn from_single(px: Price) -> Self {
        Self { ask: px, bid: px }
    }
}

#[derive(Clone, Copy)]
pub struct PositionPrices {
    pub entry: Price,
    pub exit: Price,
}
