// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream;
use quote::ToTokens;

mod input;
mod resolver;
mod file;

pub use input::ValueMacroInput;

pub fn config(item: ValueMacroInput) -> syn::Result<TokenStream> {
    Ok(resolver::run(item)?.into_token_stream())
}
