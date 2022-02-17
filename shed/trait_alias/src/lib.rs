/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

//! Procedural macro to enable trait aliases.

extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Error, ItemTraitAlias};

/// Implement a trait alias using a subtrait and blanket definition.
///
/// Trait aliases are not yet available in stable Rust, and the preview
/// version has limitations: the additional traits can only be marker
/// traits.
///
/// Allow their implementation by translating them to a different
/// mechanism.
///
/// This macro converts an item like this:
///
/// ```no_run
/// # trait Bar {} trait Baz {} trait Quux {}
/// use trait_alias::trait_alias;
///
/// #[trait_alias]
/// trait Foo = Bar + Baz + Quux;
/// ```
///
/// into a sub-trait definition and a blanket implementation:
///
/// ```no_run
/// # trait Bar {} trait Baz {} trait Quux {}
/// trait Foo: Bar + Baz + Quux {}
///
/// impl<T> Foo for T where T: Bar + Baz + Quux {}
/// ```
///
/// Note that this approach has its own drawbacks, however it should work fine
/// for simple use cases.
#[proc_macro_attribute]
pub fn trait_alias(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let _ = parse_macro_input!(attr as syn::parse::Nothing);
    let trait_alias = parse_macro_input!(item as ItemTraitAlias);

    match gen_trait_alias(trait_alias) {
        Ok(output) => output,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

fn gen_trait_alias(trait_alias: ItemTraitAlias) -> Result<TokenStream, Error> {
    let vis = trait_alias.vis;
    let ident = trait_alias.ident;
    let bounds = trait_alias.bounds;

    Ok(quote! {
        #vis trait #ident : #bounds {}
        impl<T> #ident for T where T: #bounds {}
    })
}
