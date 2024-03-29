// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//! The sync module contains various primitives of synchronization.
//! List of the primitives:
//!
//! - [`lazy`] A lazy cell that is Send + Sync, based on [`once`]
//! - [`once`] A once cell that is Send + Sync
//! - [`spinning_top`] Re-exports the spinning top Spinlock crate

mod lazy;
mod once;

pub use lazy::*;
pub use once::*;
pub use spinning_top::*;
