#![doc(html_root_url = "http://docs.rs/dyn-any-derive/0.1.0")]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_roids::{DeriveInputStructExt, FieldExt, IdentExt};
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, DeriveInput, GenericParam, Ident, Lifetime,
    LifetimeDef, Type, TypeParamBound,
};

/// Derives an implementation for the [`DynAny`] trait.
///
/// # Note
///
/// Currently only works with `struct` inputs.
///
/// # Example
///
/// ## Struct
///
/// ```
/// # use const_default::DynAny;
/// #[derive(DynAny)]
/// # #[derive(Debug, PartialEq)]
/// pub struct Color {
///     r: u8,
///     g: u8,
///     b: u8,
/// }
///
/// assert_eq!(
/// TODO: fix example
///     <Color as DynAny>::DEFAULT,
///     Color { r: 0, g: 0, b: 0 },
/// )
/// ```
///
/// ## Tuple Struct
///
/// ```
/// # use const_default::DynAny;
/// #[derive(DynAny)]
/// # #[derive(Debug, PartialEq)]
/// pub struct Vec3(f32, f32, f32);
///
/// assert_eq!(
/// TODO: fix example
///     <Vec3 as DynAny>::DEFAULT,
///     Vec3(0.0, 0.0, 0.0),
/// )
/// ```

#[proc_macro_derive(DynAny, attributes(dyn_any_derive))]
pub fn system_desc_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let fields = ast.fields();
    let struct_name = &ast.ident;
    let vis = &ast.vis;
    let generics = &ast.generics;
    let attrs = &ast.attrs;

    let static_params = replace_lifetimes(generics, "'static");
    let dyn_params = replace_lifetimes(generics, "'dyn_any");

    let old_params = &generics.params.iter().collect::<Vec<_>>();
    let foo = quote! {
        impl<'dyn_any, #(#old_params,)*> DynAny<'dyn_any> for #struct_name <#(#dyn_params,)*> {
            type Static =  #struct_name <#(#static_params,)*>;
        }
    };
    //panic!("{:?}", foo.to_string());
    TokenStream::from(foo)
}

fn replace_lifetimes(
    generics: &syn::Generics,
    replacement: &str,
) -> Vec<proc_macro2::TokenStream> {
    let params = generics
        .params
        .iter()
        .map(|param| {
            let param = match param {
                GenericParam::Lifetime(_) => {
                    GenericParam::Lifetime(LifetimeDef::new(Lifetime::new(
                        replacement,
                        Span::call_site(),
                    )))
                }
                GenericParam::Type(t) => {
                    let mut t = t.clone();
                    t.bounds.iter_mut().for_each(|bond| {
                        if let TypeParamBound::Lifetime(ref mut t) = bond {
                            *t = Lifetime::new(replacement, Span::call_site())
                        }
                    });
                    GenericParam::Type(t.clone())
                }
                c => c.clone(),
            };
            quote! {#param}
        })
        .collect::<Vec<_>>();
    params
}
