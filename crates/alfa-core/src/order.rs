/*
 * SPDX-FileCopyrightText: © 2022 The Alfa Authors <https://github.com/kyper-io/alfa/blob/913e3afe4177a9846b55c7c28c3f43a304736656/AUTHORS>
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use crate::{InstrumentId, Quantity};

// TODO: replace with trait?
pub enum Order {
    Market {
        instrument_id: InstrumentId,
        quantity: Quantity,
    },
}
