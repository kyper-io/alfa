/*
 * SPDX-FileCopyrightText: © 2022 The Alfa Authors <https://github.com/kyper-io/alfa/blob/913e3afe4177a9846b55c7c28c3f43a304736656/AUTHORS>
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

// TODO: strong typing
pub type Quantity = f64; // TODO: rename?
pub type NotionalQuantity = f64;

#[derive(Clone, Copy)]
pub struct NonZeroQuantity(Quantity);

impl NonZeroQuantity {
    #[must_use]
    pub fn new(quantity: Quantity) -> Option<Self> {
        if quantity == 0.0 {
            None
        } else {
            Some(Self(quantity))
        }
    }

    #[must_use]
    pub const fn get(self) -> Quantity {
        self.0
    }
}

#[must_use]
pub fn same_side(lhs: Quantity, rhs: Quantity) -> bool {
    lhs.signum() == rhs.signum()
}
