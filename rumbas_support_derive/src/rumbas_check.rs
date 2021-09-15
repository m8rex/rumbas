use crate::input::{InputFieldReceiver, InputVariantReceiver};
use darling::ast;
use darling::FromDeriveInput;
use quote::{quote, ToTokens};

#[derive(FromDeriveInput)]
pub struct RumbasCheckReceiver {
    ident: syn::Ident,

    /// The type's generics. You'll need these any time your trait is expected
    /// to work with types that declare generics.
    generics: syn::Generics,

    data: ast::Data<InputVariantReceiver, InputFieldReceiver>,
}

fn rumbas_check_handle_unit_struct(
    ident: &syn::Ident,
    generics: &syn::Generics,
    tokens: &mut proc_macro2::TokenStream,
) {
    let (imp, ty, wher) = generics.split_for_impl();
    tokens.extend(quote! {
            #[automatically_derived]
            impl #imp RumbasCheck for #ident #ty #wher {
                fn check(&self, _locale: &str) -> RumbasCheckResult {
                    RumbasCheckResult::empty()
                }
            }
    });
}

fn rumbas_check_handle_tuple_struct(
    fields: &ast::Fields<InputFieldReceiver>,
    ident: &syn::Ident,
    generics: &syn::Generics,
    tokens: &mut proc_macro2::TokenStream,
) {
    let (imp, ty, wher) = generics.split_for_impl();
    let field_indexes = fields
        .iter()
        .enumerate()
        .map(|(i, _f)| {
            let i = syn::Index::from(i);
            quote!(#i)
        })
        .collect::<Vec<_>>();

    tokens.extend(quote! {
            #[automatically_derived]
            impl #imp RumbasCheck for #ident #ty #wher {
                fn check(&self, locale: &str) -> RumbasCheckResult {
                    let mut result = RumbasCheckResult::empty();
                    #(
                        let mut previous_result = self.#field_indexes.check(locale);
                        previous_result.extend_path(stringify!(#field_indexes).to_string());
                        result.union(&previous_result);
                    )*
                    result
                }
            }
    });
}

fn rumbas_check_handle_struct_struct(
    fields: &ast::Fields<InputFieldReceiver>,
    ident: &syn::Ident,
    generics: &syn::Generics,
    tokens: &mut proc_macro2::TokenStream,
) {
    let (imp, ty, wher) = generics.split_for_impl();
    let field_names = fields
        .iter()
        .map(|f| f.ident.as_ref().map(|v| quote!(#v)).unwrap())
        .collect::<Vec<_>>();

    tokens.extend(quote! {
        #[automatically_derived]
        impl #imp RumbasCheck for #ident #ty #wher {
            fn check(&self, locale: &str) -> RumbasCheckResult {
                let mut result = RumbasCheckResult::empty();
                #(
                    let mut previous_result = self.#field_names.check(locale);
                    previous_result.extend_path(stringify!(#field_names).to_string());
                    result.union(&previous_result);
                )*
                result
            }
        }
    });
}

fn rumbas_check_handle_struct(
    fields: &ast::Fields<InputFieldReceiver>,
    ident: &syn::Ident,
    generics: &syn::Generics,
    tokens: &mut proc_macro2::TokenStream,
) {
    match fields.style {
        ast::Style::Struct => rumbas_check_handle_struct_struct(fields, ident, generics, tokens),
        ast::Style::Tuple => rumbas_check_handle_tuple_struct(fields, ident, generics, tokens),
        ast::Style::Unit => rumbas_check_handle_unit_struct(ident, generics, tokens),
    }
}

fn rumbas_check_handle_enum_check_variants(
    v: &[InputVariantReceiver],
    ident: &syn::Ident,
) -> Vec<proc_macro2::TokenStream> {
    v.iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            match variant.fields.style {
                ast::Style::Unit => {
                    quote! {
                        #ident::#variant_ident => RumbasCheckResult::empty()
                    }
                }
                ast::Style::Tuple => {
                    let items = variant
                        .fields
                        .fields
                        .iter()
                        .enumerate()
                        .map(|(i, _)| {
                            syn::Ident::new(
                                &format!("elem{}", i)[..],
                                proc_macro2::Span::call_site(),
                            )
                        })
                        .collect::<Vec<_>>();
                    let numbers = variant
                        .fields
                        .fields
                        .iter()
                        .enumerate()
                        .map(|(i, _)| {
                            let i = syn::Index::from(i);
                            quote!(#i)
                        })
                        .collect::<Vec<_>>();
                    quote! {
                        #ident::#variant_ident(#(#items),*) => {
                            let mut result = RumbasCheckResult::empty();
                            #(
                                let mut previous_result = #items.check(locale);
                                previous_result.extend_path(stringify!(#numbers).to_string());
                                result.union(&previous_result);
                            )*
                            result
                        }
                    }
                }
                ast::Style::Struct => {
                    let items = variant
                        .fields
                        .fields
                        .iter()
                        .map(|f| f.ident.as_ref().map(|a| quote!(#a)).unwrap())
                        .collect::<Vec<_>>();
                    quote! {
                        #ident::#variant_ident { #(#items),* } => {
                            let mut result = RumbasCheckResult::empty();
                            #(
                                let mut previous_result = #items.check(locale);
                                previous_result.extend_path(stringify!(#items).to_string());
                                result.union(&previous_result);
                            )*
                            result
                        }
                    }
                }
            }
        })
        .collect::<Vec<_>>()
}

fn rumbas_check_handle_enum(
    v: &[InputVariantReceiver],
    ident: &syn::Ident,
    generics: &syn::Generics,
    tokens: &mut proc_macro2::TokenStream,
) {
    let (imp, ty, wher) = generics.split_for_impl();

    let check_variants = rumbas_check_handle_enum_check_variants(v, ident);

    tokens.extend(quote! {
        #[automatically_derived]
        impl #imp RumbasCheck for #ident #ty #wher {
            fn check(&self, locale: &str) -> RumbasCheckResult {
                match self {
                    #(#check_variants),*
                }
            }
        }
    });
}

impl ToTokens for RumbasCheckReceiver {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let RumbasCheckReceiver {
            ref ident,
            ref generics,
            ref data,
        } = *self;

        match data {
            ast::Data::Enum(v) => rumbas_check_handle_enum(v, ident, generics, tokens),
            ast::Data::Struct(fields) => {
                rumbas_check_handle_struct(fields, ident, generics, tokens)
            }
        }
    }
}
