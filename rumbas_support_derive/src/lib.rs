extern crate darling;
extern crate proc_macro;
extern crate quote;
#[macro_use]
extern crate syn;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

mod input;
mod overwrite;

#[proc_macro_derive(Input, attributes(input))]
pub fn derive_input(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);

    let input = match input::InputReceiver::from_derive_input(&derive_input) {
        Ok(sm) => sm,
        Err(e) => panic!("error in derive(Input): {}", e),
    };

    quote!(#input).into()
}

//#[proc_macro_derive(Overwrite, attributes(input, derive, cfg, doc))]
#[proc_macro_derive(Overwrite, attributes(input))]
pub fn derive_overwrite(input: TokenStream) -> TokenStream {
    //println!("{:#?}", input);
    let derive_input = parse_macro_input!(input as syn::DeriveInput);

    let overwrite = match overwrite::OverwriteReceiver::from_derive_input(&derive_input) {
        Ok(sm) => sm,
        Err(e) => panic!("error in derive(Overwrite): {}", e),
    };

    //println!("{:#?}", overwrite);

    quote!(#overwrite).into()
}
