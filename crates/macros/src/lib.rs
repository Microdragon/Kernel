// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod config;
mod init;
mod runner;

#[proc_macro]
pub fn config(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as config::ValueMacroInput);

    match config::config(item) {
        Ok(ts) => ts.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn init(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::Item);

    match init::init(attr.into(), item) {
        Ok(ts) => ts.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

#[proc_macro]
pub fn include_runner(item: TokenStream) -> TokenStream {
    match runner::include_runner(item.into()) {
        Ok(ts) => ts.into(),
        Err(error) => error.into_compile_error().into(),
    }
}
