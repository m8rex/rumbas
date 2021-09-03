use crate::input::{InputFieldReceiver, InputVariantReceiver};
use darling::ast;
use darling::{FromDeriveInput, FromField};
use quote::{quote, ToTokens};

#[derive(Debug, FromField)]
struct OverwriteFieldReceiver {
    /// Get the ident of the field. For fields in tuple or newtype structs or
    /// enum bodies, this can be `None`.
    ident: Option<syn::Ident>,

    /// This magic field name pulls the type from the input.
    ty: syn::Type,
}

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(input))]
#[darling(forward_attrs)]
pub struct OverwriteReceiver {
    /// The struct ident.
    ident: syn::Ident,

    /// The type's generics. You'll need these any time your trait is expected
    /// to work with types that declare generics.
    generics: syn::Generics,

    /// Receives the body of the struct or enum. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    data: ast::Data<InputVariantReceiver, InputFieldReceiver>,
    attrs: Vec<syn::Attribute>,

    /// I guess we can't get other derives into `attrs` so we have to create our
    /// own derive list.
    //derive: darling::util::PathList,

    #[darling(rename = "name")]
    input_name: String,
}
impl ToTokens for OverwriteReceiver {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let OverwriteReceiver {
            ref ident,
            ref generics,
            ref data,
            ref attrs,
            ref input_name,
            //ref derive,
        } = *self;

        println!("{:?}", ident);
        let derive_attrs = attrs
            .iter()
            .filter(|a| a.path.is_ident("derive"))
            .collect::<Vec<_>>();
        println!("{:#?}", derive_attrs);
        let to_derive = derive_attrs
            .iter()
            .flat_map(|a| {
                let group = a.tokens.clone().into_iter().next().expect("Invalid derive");
                match group {
                    proc_macro2::TokenTree::Group(g) => g
                        .stream()
                        .into_iter()
                        .flat_map(|tt| {
                            if let proc_macro2::TokenTree::Ident(i) = tt {
                                vec![i]
                            } else {
                                vec![]
                            }
                        })
                        .collect::<Vec<_>>(),
                    _ => panic!("Invalid derive"),
                }
            })
            .collect::<Vec<_>>();
        println!("{:?}", to_derive);

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

    tokens.extend(quote! {
        #[automatically_derived]
        impl #imp Overwrite<#input_ident #ty> for #input_ident #ty #wher {
            fn overwrite(&mut self, other: &Self){
                #(self.#field_names.overwrite(&other.#field_names);)*
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
    v: &Vec<InputVariantReceiver>,
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
                    #(#overwrite_variants),*
                    _ => ()
                }
            }
        }
    });
}

fn overwrite_handle_enum_variants(
    v: &Vec<InputVariantReceiver>,
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn used_to_debug() {
        let derive_input = syn::parse_str(
            r#"/// Some comment
            #[derive(Clone, Input, Overwrite)]
            #[input(name="test")]
            pub struct A {
                /// Hi
                d: bool
            }"#,
        )
        .unwrap();
        let parsed = OverwriteReceiver::from_derive_input(&derive_input).unwrap();
        assert_eq!(false, true);
    }
}
