use crate::input::{InputFieldReceiver, InputVariantReceiver};
use darling::ast;
use darling::FromDeriveInput;
use quote::{quote, ToTokens};

#[derive(FromDeriveInput)]
pub struct ExamplesReceiver {
    ident: syn::Ident,

    /// The type's generics. You'll need these any time your trait is expected
    /// to work with types that declare generics.
    generics: syn::Generics,

    data: ast::Data<InputVariantReceiver, InputFieldReceiver>,
}

fn handle_unit_struct(
    ident: &syn::Ident,
    generics: &syn::Generics,
    tokens: &mut proc_macro2::TokenStream,
) {
    let (imp, ty, wher) = generics.split_for_impl();
    tokens.extend(quote! {
            #[automatically_derived]
            impl #imp Examples for #ident #ty #wher {
                fn examples() -> Vec<Self> {
                    vec![Self]
                }
            }
    });
}

fn tuple_body(
    fields: &ast::Fields<InputFieldReceiver>,
    type_name: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
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
    quote! {
                    let tuple_data = <(#(#field_types),*)>::examples();
                    tuple_data.into_iter().map(|t| #type_name(#(t.#field_indexes),*) ).collect::<Vec<_>>()
    }
}

fn handle_tuple_struct(
    fields: &ast::Fields<InputFieldReceiver>,
    ident: &syn::Ident,
    generics: &syn::Generics,
    tokens: &mut proc_macro2::TokenStream,
) {
    let (imp, ty, wher) = generics.split_for_impl();
    let body = tuple_body(fields, quote!(Self));

    tokens.extend(quote! {
            #[automatically_derived]
            impl #imp Examples for #ident #ty #wher {
                fn examples() -> Vec<Self> {
                    #body
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
                    f.ident.as_ref().map(|v| v.to_string()).unwrap(),
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
                    f.ident.as_ref().map(|v| v.to_string()).unwrap(),
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
                    f.ident.as_ref().map(|v| v.to_string()).unwrap(),
                )[..],
                f.ident.as_ref().map(|f| f.span()).unwrap(),
            )
        })
        .collect::<Vec<_>>();
    quote! {
        #(
            let mut #field_name_examples = <#field_types>::examples();
        )*
        let mut max_examples = 0;
        #(
            max_examples = std::cmp::max(max_examples, #field_name_examples.len());
        )*
        #(
            while #field_name_examples.len() < max_examples {
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
    ident: &syn::Ident,
    generics: &syn::Generics,
    tokens: &mut proc_macro2::TokenStream,
) {
    let (imp, ty, wher) = generics.split_for_impl();
    let body = struct_body(fields, quote!(Self));

    tokens.extend(quote! {
        #[automatically_derived]
        impl #imp Examples for #ident #ty #wher {
            fn examples() -> Vec<Self> {
                #body
            }
        }
    });
}

fn handle_struct(
    fields: &ast::Fields<InputFieldReceiver>,
    ident: &syn::Ident,
    generics: &syn::Generics,
    tokens: &mut proc_macro2::TokenStream,
) {
    match fields.style {
        ast::Style::Struct => handle_struct_struct(fields, ident, generics, tokens),
        ast::Style::Tuple => handle_tuple_struct(fields, ident, generics, tokens),
        ast::Style::Unit => handle_unit_struct(ident, generics, tokens),
    }
}

fn handle_enum_check_variants(
    v: &[InputVariantReceiver],
    ident: &syn::Ident,
) -> Vec<proc_macro2::TokenStream> {
    v.iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            match variant.fields.style {
                ast::Style::Unit => {
                    quote! {
                        vec![#ident::#variant_ident]
                    }
                }
                ast::Style::Tuple => {
                    tuple_body(&variant.fields, quote!(#ident::#variant_ident))
                    /*
                    let field_types = variant
                        .fields
                        .fields
                        .iter()
                        .map(|f| {
                            let ty = f.ty.clone();
                            quote!(#ty)
                        })
                        .collect::<Vec<_>>();
                    let field_indexes = variant
                        .fields
                        .fields
                        .iter()
                        .enumerate()
                        .map(|(i, _)| {
                            let i = syn::Index::from(i);
                            quote!(#i)
                        })
                        .collect::<Vec<_>>(); */
                    /*quote! {
                        let tuple_data = <(#(#field_types),*)>::examples();
                        tuple_data.into_iter().map(|t| #ident::#variant_ident(#(t.#field_indexes),*) ).collect()
                    }*/
                }
                ast::Style::Struct => struct_body(&variant.fields, quote!(#ident::#variant_ident)),
            }
        })
        .collect::<Vec<_>>()
}

fn handle_enum(
    v: &[InputVariantReceiver],
    ident: &syn::Ident,
    generics: &syn::Generics,
    tokens: &mut proc_macro2::TokenStream,
) {
    let (imp, ty, wher) = generics.split_for_impl();

    let check_variants = handle_enum_check_variants(v, ident);

    tokens.extend(quote! {
        #[automatically_derived]
        impl #imp Examples for #ident #ty #wher {
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
        } = *self;

        match data {
            ast::Data::Enum(v) => handle_enum(v, ident, generics, tokens),
            ast::Data::Struct(fields) => handle_struct(fields, ident, generics, tokens),
        }
    }
}
