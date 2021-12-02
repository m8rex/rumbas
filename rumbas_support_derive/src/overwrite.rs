use crate::input::{InputFieldReceiver, InputVariantReceiver};
use darling::ast;
use darling::FromDeriveInput;
use quote::{quote, ToTokens};

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(input))]
#[darling(forward_attrs)]
pub struct OverwriteReceiver {
    ident: syn::Ident,

    /// The type's generics. You'll need these any time your trait is expected
    /// to work with types that declare generics.
    generics: syn::Generics,

    data: ast::Data<InputVariantReceiver, InputFieldReceiver>,

    #[darling(rename = "name")]
    input_name: String,

    #[darling(default)]
    test: bool,
}
impl ToTokens for OverwriteReceiver {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let OverwriteReceiver {
            ref ident,
            ref generics,
            ref data,
            ref input_name,
            test: _,
        } = *self;

        let input_ident = syn::Ident::new(&input_name, ident.span());

        match data {
            ast::Data::Enum(v) => overwrite_handle_enum(v, &input_ident, generics, tokens),
            ast::Data::Struct(fields) => {
                overwrite_handle_struct(fields, &input_ident, generics, tokens)
            }
        }
    }
}

fn overwrite_handle_unit_struct(
    input_ident: &syn::Ident,
    generics: &syn::Generics,
    tokens: &mut proc_macro2::TokenStream,
) {
    let (imp, ty, wher) = generics.split_for_impl();
    tokens.extend(quote! {
            #[automatically_derived]
            impl #imp Overwrite<#input_ident #ty> for #input_ident #ty #wher {
                fn overwrite(&mut self, _other: &Self){

                }
            }
    });
}

fn overwrite_handle_tuple_struct(
    fields: &ast::Fields<InputFieldReceiver>,
    input_ident: &syn::Ident,
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
            impl #imp Overwrite<#input_ident #ty> for #input_ident #ty #wher {
                fn overwrite(&mut self, other: &Self){
                    #(self.#field_indexes.overwrite(&other.#field_indexes);)*
                }
            }
    });
}

fn overwrite_handle_struct_struct(
    fields: &ast::Fields<InputFieldReceiver>,
    input_ident: &syn::Ident,
    generics: &syn::Generics,
    tokens: &mut proc_macro2::TokenStream,
) {
    let (imp, ty, wher) = generics.split_for_impl();
    let field_names = fields
        .iter()
        .map(|f| f.ident.as_ref().map(|v| quote!(#v)).unwrap())
        .collect::<Vec<_>>();

    let enum_input_ident = syn::Ident::new(
        &format!("{}Enum", input_ident.to_string())[..],
        input_ident.span(),
    );

    tokens.extend(quote! {
        #[automatically_derived]
        impl #imp Overwrite<#input_ident #ty> for #input_ident #ty #wher {
            fn overwrite(&mut self, other: &Self){
                #(self.#field_names.overwrite(&other.#field_names);)*
            }
        }
        #[automatically_derived]
        impl #imp Overwrite<#enum_input_ident #ty> for #input_ident #ty #wher {
            fn overwrite(&mut self, other: &#enum_input_ident){
               self.overwrite(&other.0)
            }
        }
        #[automatically_derived]
        impl #imp Overwrite<#input_ident #ty> for #enum_input_ident #ty #wher {
            fn overwrite(&mut self, other: &#input_ident){
                self.0.overwrite(other)
            }
        }
        #[automatically_derived]
        impl #imp Overwrite<#enum_input_ident #ty> for #enum_input_ident #ty #wher {
            fn overwrite(&mut self, other: &Self){
                self.0.overwrite(&other.0)
            }
        }
    });
}

fn overwrite_handle_struct(
    fields: &ast::Fields<InputFieldReceiver>,
    input_ident: &syn::Ident,
    generics: &syn::Generics,
    tokens: &mut proc_macro2::TokenStream,
) {
    match fields.style {
        ast::Style::Struct => overwrite_handle_struct_struct(fields, input_ident, generics, tokens),
        ast::Style::Tuple => overwrite_handle_tuple_struct(fields, input_ident, generics, tokens),
        ast::Style::Unit => overwrite_handle_unit_struct(input_ident, generics, tokens),
    }
}

fn overwrite_handle_enum(
    v: &[InputVariantReceiver],
    input_ident: &syn::Ident,
    generics: &syn::Generics,
    tokens: &mut proc_macro2::TokenStream,
) {
    let (imp, ty, wher) = generics.split_for_impl();

    let overwrite_variants = overwrite_handle_enum_variants(v, input_ident);

    tokens.extend(quote! {
        #[automatically_derived]
        impl #imp Overwrite<#input_ident #ty> for #input_ident #ty #wher {
            fn overwrite(&mut self, other: &Self){
                match (self, other) {
                    #(#overwrite_variants,)*
                    _ => ()
                }
            }
        }
    });
}

fn overwrite_handle_enum_variants(
    v: &[InputVariantReceiver],
    input_ident: &syn::Ident,
) -> Vec<proc_macro2::TokenStream> {
    v.iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            match variant.fields.style {
                ast::Style::Unit => {
                    quote! {
                        (#input_ident::#variant_ident, #input_ident::#variant_ident) => ()
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
                    let items2 = variant
                        .fields
                        .fields
                        .iter()
                        .enumerate()
                        .map(|(i, _)| {
                            syn::Ident::new(
                                &format!("other_elem{}", i)[..],
                                proc_macro2::Span::call_site(),
                            )
                        })
                        .collect::<Vec<_>>();
                    quote! {
                        (#input_ident::#variant_ident(#(#items),*), #input_ident::#variant_ident(#(#items2),*), ) => {
                            #(#items.overwrite(#items2));*
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
                    let items2 = variant
                        .fields
                        .fields
                        .iter()
                        .enumerate()
                        .map(|(i, _)| {
                            syn::Ident::new(
                                &format!("other_elem{}", i)[..],
                                proc_macro2::Span::call_site(),
                            )
                        })
                        .collect::<Vec<_>>();
                    quote! {
                        (#input_ident::#variant_ident { #(#items),* }, #input_ident::#variant_ident { #(#items:#items2),* }) => {
                            #(#items.overwrite(#items2));*
                        }
                    }
                }
            }
        })
        .collect::<Vec<_>>()
}
