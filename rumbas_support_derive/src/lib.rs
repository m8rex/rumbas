extern crate darling;
extern crate proc_macro;
extern crate quote;
#[macro_use]
extern crate syn;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

mod examples;
mod input;
mod overwrite;
mod rumbas_check;

#[proc_macro_derive(Input, attributes(input))]
pub fn derive_input(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);

    let input = match input::InputReceiver::from_derive_input(&derive_input) {
        Ok(sm) => sm,
        Err(e) => panic!("error in derive(Input): {}", e),
    };

    quote!(#input).into()
}

#[proc_macro_derive(Overwrite, attributes(input))]
pub fn derive_overwrite(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);

    let overwrite = match overwrite::OverwriteReceiver::from_derive_input(&derive_input) {
        Ok(sm) => sm,
        Err(e) => panic!("error in derive(Overwrite): {}", e),
    };

    quote!(#overwrite).into()
}

#[proc_macro_derive(RumbasCheck)]
pub fn derive_rumbas_check(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);

    let rumbas_check = match rumbas_check::RumbasCheckReceiver::from_derive_input(&derive_input) {
        Ok(sm) => sm,
        Err(e) => panic!("error in derive(RumbasCheck): {}", e),
    };

    quote!(#rumbas_check).into()
}

#[proc_macro_derive(Examples, attributes(input))]
pub fn derive_examples(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);

    let examples = match examples::ExamplesReceiver::from_derive_input(&derive_input) {
        Ok(sm) => sm,
        Err(e) => panic!("error in derive(Examples): {}", e),
    };

    quote!(#examples).into()
}

#[proc_macro]
pub fn impl_examples_for_tuple(input: TokenStream) -> TokenStream {
    let ty: syn::Type = syn::parse(input).expect("Please pass a tuple type to impl_examples");
    match ty {
        syn::Type::Tuple(t) => {
            let res = examples::impl_for_tuple(t);
            quote!(#res).into()
        }
        _ => panic!("error in impl_examples_for_tuple: only tuples are supported"),
    }
}
