use crate::input::{InputFieldReceiver, InputVariantReceiver};
use crate::syn::spanned::Spanned;
use darling::ast;
use darling::FromDeriveInput;
use quote::{quote, ToTokens};

#[derive(FromDeriveInput)]
#[darling(attributes(input))]
pub struct ExamplesReceiver {
    ident: syn::Ident,

    /// The type's generics. You'll need these any time your trait is expected
    /// to work with types that declare generics.
    generics: syn::Generics,

    data: ast::Data<InputVariantReceiver, InputFieldReceiver>,

    #[darling(default)]
    test: bool,

    #[darling(rename = "name")]
    input_name: String,
}

fn handle_unit_struct(
    input_ident: &syn::Ident,
    generics: &syn::Generics,
    tokens: &mut proc_macro2::TokenStream,
) {
    let (imp, ty, wher) = generics.split_for_impl();
    tokens.extend(quote! {
            #[automatically_derived]
            impl #imp Examples for #input_ident #ty #wher {
                fn examples() -> Vec<Self> {
                    vec![Self]
                }
            }
    });
}

fn handle_tuple_struct(
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
    let field_types = fields
        .iter()
        .map(|f| {
            let ty = f.ty.clone();
            quote!(#ty)
        })
        .collect::<Vec<_>>();

    tokens.extend(quote! {
            #[automatically_derived]
            impl #imp Examples for #input_ident #ty #wher {
                fn examples() -> Vec<Self> {
                    let tuple_data = <(#(ValueType<<#field_types as InputInverse>::Input>,)*)>::examples();
                    tuple_data.into_iter().map(|t| Self(#(t.#field_indexes),*) ).collect::<Vec<_>>()
                }
            }
    });
}

fn struct_body(
    fields: &ast::Fields<InputFieldReceiver>,
    type_name: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let field_names = fields
        .iter()
        .map(|f| f.ident.as_ref().map(|v| quote!(#v)).unwrap())
        .collect::<Vec<_>>();
    let field_types = fields
        .iter()
        .map(|f| {
            let ty = f.ty.clone();
            quote!(#ty)
        })
        .collect::<Vec<_>>();
    let field_name_examples = fields
        .iter()
        .map(|f| {
            syn::Ident::new(
                &format!(
                    "{}_examples",
                    f.ident.as_ref().map(|v| clean_ident_name(v)).unwrap(),
                )[..],
                f.ident.as_ref().map(|f| f.span()).unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let field_name_iterators = fields
        .iter()
        .map(|f| {
            syn::Ident::new(
                &format!(
                    "{}_iterator",
                    f.ident.as_ref().map(|v| clean_ident_name(v)).unwrap(),
                )[..],
                f.ident.as_ref().map(|f| f.span()).unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let field_name_options = fields
        .iter()
        .map(|f| {
            syn::Ident::new(
                &format!(
                    "{}_option",
                    f.ident.as_ref().map(|v| clean_ident_name(v)).unwrap(),
                )[..],
                f.ident.as_ref().map(|f| f.span()).unwrap(),
            )
        })
        .collect::<Vec<_>>();
    quote! {
        #(
            let mut #field_name_examples = <Value<<#field_types as InputInverse>::Input>>::examples();
        )*
        let mut max_examples_number = 0;
        #(
            max_examples_number = std::cmp::max(max_examples_number, #field_name_examples.len());
        )*
        #(
            while #field_name_examples.len() < max_examples_number {
                #field_name_examples.extend(<Value<<#field_types as InputInverse>::Input>>::examples());
            }
            let mut #field_name_iterators = #field_name_examples.into_iter();
        )*

        let mut result = Vec::new();
        loop {
            #(
                let #field_name_options = #field_name_iterators.next();
                if #field_name_options.is_none() {
                    break;
                }
            )*
            result.push(
                #type_name {
                    #(
                       #field_names: #field_name_options.unwrap()
                    ),*
                }
            )
        }
        result
    }
}

fn handle_struct_struct(
    fields: &ast::Fields<InputFieldReceiver>,
    input_ident: &syn::Ident,
    generics: &syn::Generics,
    tokens: &mut proc_macro2::TokenStream,
) {
    let (imp, ty, wher) = generics.split_for_impl();
    let body = struct_body(fields, quote!(Self));

    let enum_input_ident = syn::Ident::new(
        &format!("{}Enum", input_ident.to_string())[..],
        input_ident.span(),
    );

    tokens.extend(quote! {
        #[automatically_derived]
        impl #imp Examples for #input_ident #ty #wher {
            fn examples() -> Vec<Self> {
                #body
            }
        }
        #[automatically_derived]
        impl #imp Examples for #enum_input_ident #ty #wher {
            fn examples() -> Vec<Self> {
                <#input_ident>::examples().into_iter().filter_map(|e| std::convert::TryInto::try_into(e).ok()).collect()
            }
        }
    });
}

fn handle_struct(
    fields: &ast::Fields<InputFieldReceiver>,
    input_ident: &syn::Ident,
    generics: &syn::Generics,
    tokens: &mut proc_macro2::TokenStream,
) {
    match fields.style {
        ast::Style::Struct => handle_struct_struct(fields, input_ident, generics, tokens),
        ast::Style::Tuple => handle_tuple_struct(fields, input_ident, generics, tokens),
        ast::Style::Unit => handle_unit_struct(input_ident, generics, tokens),
    }
}

fn handle_enum_check_variants(
    v: &[InputVariantReceiver],
    input_ident: &syn::Ident,
) -> Vec<proc_macro2::TokenStream> {
    v.iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            match variant.fields.style {
                ast::Style::Unit => {
                    quote! {
                        vec![#input_ident::#variant_ident]
                    }
                }
                ast::Style::Tuple => {
                    let field_indexes = variant.fields.fields
                        .iter()
                        .enumerate()
                        .map(|(i, _f)| {
                            let i = syn::Index::from(i);
                            quote!(#i)
                        })
                        .collect::<Vec<_>>();
                    let field_types = variant.fields.fields
                        .iter()
                        .map(|f| {
                            let ty = f.ty.clone();
                            quote!(#ty)
                        })
                        .collect::<Vec<_>>();
                    quote! {
                        let tuple_data = <(#(<#field_types as InputInverse>::EnumInput,)*)>::examples();
                        tuple_data.into_iter().map(|t| #input_ident::#variant_ident(#(t.#field_indexes),*) ).collect::<Vec<_>>()
                    }
                },
                ast::Style::Struct => {
                    struct_body(&variant.fields, quote!(#input_ident::#variant_ident))
                }
            }
        })
        .collect::<Vec<_>>()
}

fn handle_enum(
    v: &[InputVariantReceiver],
    input_ident: &syn::Ident,
    generics: &syn::Generics,
    tokens: &mut proc_macro2::TokenStream,
) {
    let (imp, ty, wher) = generics.split_for_impl();

    let check_variants = handle_enum_check_variants(v, input_ident);

    tokens.extend(quote! {
        #[automatically_derived]
        impl #imp Examples for #input_ident #ty #wher {
            fn examples() -> Vec<Self> {
                let mut all : Vec<Self> = Vec::new();
                #(
                    all.extend({#check_variants}.into_iter());
                )*
                all
            }
        }
    });
}

impl ToTokens for ExamplesReceiver {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ExamplesReceiver {
            ref ident,
            ref generics,
            ref data,
            test,
            ref input_name,
        } = *self;

        let input_ident = syn::Ident::new(&input_name, ident.span());

        match data {
            ast::Data::Enum(v) => handle_enum(v, &input_ident, generics, tokens),
            ast::Data::Struct(fields) => handle_struct(fields, &input_ident, generics, tokens),
        }

        if test {
            let mod_ident = syn::Ident::new(
                &format!("examples_{}", input_ident.to_string().to_lowercase())[..],
                ident.span(),
            );

            tokens.extend(quote! {
                #[cfg(test)]
                mod #mod_ident {
                    use super::*;
                    use rumbas_support::example::Examples;
                    #[test]
                    fn compile_examples() {
                        for example in <#input_ident>::examples().into_iter() {
                            let item = serde_yaml::to_string(&example);
                            assert!(item.is_ok());
                            let item = item.unwrap();
                            let parsed: Result<#input_ident, _> = serde_yaml::from_str(&item[..]);
                            if let Err(ref e) = parsed {
                                if "No field is set to a not-none value." == &e.to_string()[..] {
                                    continue;
                                }
                                println!("Input: {:?}", item);
                                println!("Error: {:?}", e);
                            }
                            assert!(parsed.is_ok());
                            assert_eq!(example, parsed.unwrap());
                            insta::with_settings!({sort_maps => true}, {
                                insta::assert_yaml_snapshot!(&example);
                            });
                        }
                    }
                }
            });
        }
    }
}

pub fn impl_for_tuple(tup: syn::TypeTuple) -> proc_macro2::TokenStream {
    let field_types = tup.elems.iter().map(|e| quote!(#e)).collect::<Vec<_>>();
    let field_name_examples = tup
        .elems
        .iter()
        .enumerate()
        .map(|(i, t)| syn::Ident::new(&format!("examples{}", i,)[..], t.span()))
        .collect::<Vec<_>>();
    let field_name_iterators = tup
        .elems
        .iter()
        .enumerate()
        .map(|(i, t)| syn::Ident::new(&format!("iterator{}", i,)[..], t.span()))
        .collect::<Vec<_>>();
    let field_name_options = tup
        .elems
        .iter()
        .enumerate()
        .map(|(i, t)| syn::Ident::new(&format!("option{}", i,)[..], t.span()))
        .collect::<Vec<_>>();
    quote! {
            #[automatically_derived]
            impl <#(#field_types: Examples,)*> Examples for (#(#field_types,)*) {
                fn examples() -> Vec<Self> {
                    #(
                        let mut #field_name_examples = <#field_types>::examples();
                    )*
                    let mut max_examples_number = 0;
                    #(
                        max_examples_number = std::cmp::max(max_examples_number, #field_name_examples.len());
                    )*
                    #(
                        while #field_name_examples.len() < max_examples_number {
                            #field_name_examples.extend(<#field_types>::examples());
                        }
                        let mut #field_name_iterators = #field_name_examples.into_iter();
                    )*

                    let mut result = Vec::new();
                    loop {
                        #(
                            let #field_name_options = #field_name_iterators.next();
                            if #field_name_options.is_none() {
                                break;
                            }
                        )*
                        result.push(
                            (
                                #(
                                   #field_name_options.unwrap(),
                                )*
                    )
                        )
                    }
                    result
            }
        }
    }
}

fn clean_ident_name(ident: &syn::Ident) -> String {
    let mut s = ident.to_string();
    if s.starts_with("r#") {
        s.split_off(2)
    } else {
        s
    }
}
