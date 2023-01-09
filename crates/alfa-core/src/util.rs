/*
 * SPDX-FileCopyrightText: © 2022 The Alfa Authors <https://github.com/kyper-io/alfa/blob/913e3afe4177a9846b55c7c28c3f43a304736656/AUTHORS>
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Percent {
    pub multiplier: f64,
}

impl Percent {
    fn new(pct: f64) -> Self {
        Self {
            multiplier: pct / 100.0,
        }
    }
}

pub type NotionalPercent = Percent;
