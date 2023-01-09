/*
 * SPDX-FileCopyrightText: © 2022 The Alfa Authors <https://github.com/kyper-io/alfa/blob/913e3afe4177a9846b55c7c28c3f43a304736656/AUTHORS>
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use serde::{Deserialize, Serialize};

use crate::{Cash, NotionalPercent};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Commission {
    Fixed(Cash),
    FixedPerUnit(Cash),
    FixedMakerTaker {
        maker: NotionalPercent,
        taker: NotionalPercent,
    },
}

pub struct MakerTaker<T> {
    pub maker: T,
    pub taker: T,
}

impl<T: Clone> MakerTaker<T> {
    pub fn from_single(val: T) -> Self {
        Self {
            maker: val.clone(),
            taker: val,
        }
    }
}

impl<T> MakerTaker<T> {
    pub fn map<F, U>(&self, mut f: F) -> MakerTaker<U>
    where
        F: FnMut(&T) -> U,
    {
        MakerTaker {
            maker: f(&self.maker),
            taker: f(&self.taker),
        }
    }
}
