#![doc(html_root_url = "http://docs.rs/dyn-any-derive/0.1.0")]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_roids::{DeriveInputStructExt, FieldExt, IdentExt};
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, Ident, Lifetime, Type};

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

    let variants = fields
        .iter()
        .map(|field| {
            let ident = field.ident.clone();
            let vis = field.vis.clone();
            let ty = field.ty.clone();
            let ty = match ty {
                Type::Reference(r) => {
                    let mut r = r;
                    r.lifetime =
                        Some(Lifetime::new("'static", Span::call_site()));
                    Type::Reference(r)
                }
                t => t,
            };
            if let Some(ident) = ident {
                quote! {#vis #ident: #ty}
            } else {
                quote! {#vis #ty}
            }
        })
        .collect::<Vec<_>>();
    let tuple_struct = fields.iter().any(|field| field.ident.is_none());

    let fields = if tuple_struct {
        quote! {
            ( #(#variants,)* )
        }
    } else {
        quote! {
             {   #(#variants,)* }
        }
    };
    let foo = quote! {
        #(#attrs )*
        #vis struct #struct_name #generics #fields
    };
    //panic!("{:?}", foo.to_string());
    TokenStream::from(foo)
}
