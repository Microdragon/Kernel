// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{parse_quote, Error, Ident, Item, ItemFn, ItemStatic, StaticMutability, Type};

pub fn init(attr: TokenStream, item: Item) -> syn::Result<TokenStream> {
    match item {
        Item::Static(st) => init_static(attr, st),
        Item::Fn(func) => init_fn(attr, func),
        _ => Err(Error::new_spanned(
            item,
            "the `#[init]` attribute may only be used on functions and static items",
        )),
    }
}

fn init_fn(attr: TokenStream, mut func: ItemFn) -> syn::Result<TokenStream> {
    if !attr.is_empty() {
        return Err(Error::new_spanned(
            attr,
            "the `#[init]` attribute on functions does not take any parameters",
        ));
    }

    if let Some(attr) = func
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("link_section"))
    {
        return Err(Error::new_spanned(
            attr,
            "the function already has a `#[link_section]` attribute",
        ));
    }

    func.attrs
        .push(parse_quote!(#[link_section = ".init.text"]));

    Ok(func.into_token_stream())
}

fn init_static(attr: TokenStream, mut st: ItemStatic) -> syn::Result<TokenStream> {
    let section = if attr.is_empty() {
        StaticSection::from_static_item(&st)
    } else {
        syn::parse2(attr)?
    };

    if let Some(attr) = st
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("link_section"))
    {
        return Err(Error::new_spanned(
            attr,
            "the static item already has a `#[link_section]` attribute",
        ));
    }

    let section = section.section_name();
    st.attrs.push(parse_quote!(#[link_section = #section]));

    Ok(st.into_token_stream())
}

#[derive(Clone, Copy)]
enum StaticSection {
    Immutable,
    Mutable,
    Cell,
}

impl StaticSection {
    pub fn from_static_item(st: &ItemStatic) -> Self {
        if let Type::Path(ty) = st.ty.as_ref() {
            if let Some(last) = ty.path.segments.last() {
                if last.ident == "InitCell" {
                    return StaticSection::Cell;
                }
            }
        }

        if matches!(st.mutability, StaticMutability::Mut(_)) {
            StaticSection::Mutable
        } else {
            StaticSection::Immutable
        }
    }

    pub fn section_name(self) -> &'static str {
        match self {
            StaticSection::Immutable => ".init.rodata",
            StaticSection::Mutable => ".init.data",
            StaticSection::Cell => ".init.cell",
        }
    }
}

impl Parse for StaticSection {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;

        if ident == "immutable" {
            Ok(StaticSection::Immutable)
        } else if ident == "mutable" {
            Ok(StaticSection::Mutable)
        } else if ident == "cell" {
            Ok(StaticSection::Cell)
        } else {
            Err(Error::new_spanned(
                ident,
                "expected one of 'immutable', 'mutable' or 'cell'",
            ))
        }
    }
}
