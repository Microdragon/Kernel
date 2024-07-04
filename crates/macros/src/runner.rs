// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream;
use quote::quote;
use syn::Error;

pub fn include_runner(item: TokenStream) -> syn::Result<TokenStream> {
    if !item.is_empty() {
        return Err(Error::new_spanned(
            item,
            "the `include_runner!` macro does not take any arguments",
        ));
    }

    if let Ok(runner) = std::env::var("MICRODRAGON_RUNNER") {
        Ok(quote! {
            include!(#runner);
        })
    } else {
        Ok(quote! {
            fn run_modules(interface: &ModuleInterface) {}
        })
    }
}
