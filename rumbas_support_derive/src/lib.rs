#[macro_use]
extern crate darling;
extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use darling::ast;
use darling::{FromDeriveInput, FromField, FromVariant};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};

#[derive(Debug, FromField)]
struct InputFieldReceiver {
    /// Get the ident of the field. For fields in tuple or newtype structs or
    /// enum bodies, this can be `None`.
    ident: Option<syn::Ident>,

    /// This magic field name pulls the type from the input.
    ty: syn::Type,
}

#[derive(Debug, FromVariant)]
struct InputVariantReceiver {
    /// The identifier of the passed-in variant
    ident: syn::Ident,
    // The fields associated with the variant
    fields: ast::Fields<InputFieldReceiver>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(input))]
#[darling(forward_attrs(doc, derive))]
struct InputReceiver {
    /// The struct ident.
    ident: syn::Ident,

    /// The type's generics. You'll need these any time your trait is expected
    /// to work with types that declare generics.
    generics: syn::Generics,

    /// Receives the body of the struct or enum. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    data: ast::Data<InputVariantReceiver, InputFieldReceiver>,
    attrs: Vec<syn::Attribute>,

    #[darling(rename = "name")]
    input_name: String,
}

#[proc_macro_derive(Input, attributes(input))]
pub fn derive_input(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);

    let machine = match InputReceiver::from_derive_input(&derive_input) {
        Ok(sm) => sm,
        Err(e) => panic!("error in derive(Input): {}", e),
    };

    quote!(#machine).into()
}
impl ToTokens for InputReceiver {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let InputReceiver {
            ref ident,
            ref generics,
            ref data,
            ref attrs,
            ref input_name,
        } = *self;

        /*let derive_attrs = attrs
            .iter()
            .filter(|a| a.path.is_ident("derive"))
            .collect::<Vec<_>>();
        eprintln!("{:?}", derive_attrs);*/

        let (imp, ty, wher) = generics.split_for_impl();

        let input_ident = syn::Ident::new(&input_name, ident.span());

        match data {
            ast::Data::Enum(v) => {
                let variant_names = v
                    .iter()
                    .map(|variant| {
                        let ident = &variant.ident;
                        quote! {#ident}
                    })
                    .collect::<Vec<_>>();

                let input_variants = v.iter().map(|variant| {
                    let input_type_tys = variant.fields.fields
                        .iter()
                        .enumerate()
                        .map(|(_i, f)| {
                            // This works with named or indexed fields, so we'll fall back to the index so we can
                            // write the output as a key-value pair.
                            match &f.ty {
                                syn::Type::Path(p) => {
                                    let ident_opt = p.path.get_ident();
                                    if let Some(ident) = ident_opt {
                                        ident.to_owned()
                                    } else {
                                        panic!(
                                            "{:?} is not a valid type for an Input struct.",
                                            p
                                        )
                                    }
                                }
                                _ => panic!(
                                    "{:?} is not a valid type for an Input struct.",
                                    f.ty
                                ),
                            }
                            //f.ty.clone()
                        })
                    .collect::<Vec<_>>();
                    let variant_ident = &variant.ident;
                    match variant.fields.style {
                        ast::Style::Unit => {
                            quote!{
                                #variant_ident
                            }
                        }
                        ast::Style::Tuple => {
                            quote!{
                                #variant_ident(#(Value<<#input_type_tys as InputInverse>::Input>),*)
                            }
                        }
                        ast::Style::Struct => {
                            let items = variant.fields.fields.iter().map(|f| f.ident.as_ref().map(|a|  quote!(#a)).unwrap()).collect::<Vec<_>>();
                            quote!{
                                #variant_ident {
                                    #(
                                    #items: Value<<#input_type_tys as InputInverse>::Input>
                                    ),*
                                }
                            }
                        }
                    }
                }).collect::<Vec<_>>();

                let to_normal_variants = v.iter().map(|variant| {
                    let variant_ident = &variant.ident;
                    match variant.fields.style {
                        ast::Style::Unit => {
                            quote!{
                                #input_ident::#variant_ident => #ident::#variant_ident
                            }
                        }
                        ast::Style::Tuple => {
                            let items = variant.fields.fields.iter().enumerate().map(|(i,_)| syn::Ident::new(&format!("elem{}",i)[..], proc_macro2::Span::call_site())).collect::<Vec<_>>();
                            quote!{
                                #input_ident::#variant_ident(#(#items),*) => #ident::#variant_ident(#(#items.to_normal()),*)
                            }
                        }
                        ast::Style::Struct => {
                            let items = variant.fields.fields.iter().map(|f| f.ident.as_ref().map(|a|  quote!(#a)).unwrap()).collect::<Vec<_>>();
                            quote!{
                                #input_ident::#variant_ident { #(#items),* } => #ident::#variant_ident { #(#items: #items.to_normal()),* }
                            }
                        }
                    }
                }).collect::<Vec<_>>();

                let from_normal_variants = v.iter().map(|variant| {
                    let variant_ident = &variant.ident;
                    match variant.fields.style {
                        ast::Style::Unit => {
                            quote!{
                                #ident::#variant_ident => #input_ident::#variant_ident
                            }
                        }
                        ast::Style::Tuple => {
                            let items = variant.fields.fields.iter().enumerate().map(|(i,_)| syn::Ident::new(&format!("elem{}",i)[..], proc_macro2::Span::call_site())).collect::<Vec<_>>();
                            quote!{
                                #ident::#variant_ident(#(#items),*) => #input_ident::#variant_ident(#(Input::from_normal(#items)),*)
                            }
                        }
                        ast::Style::Struct => {
                            let items = variant.fields.fields.iter().map(|f| f.ident.as_ref().map(|a|  quote!(#a)).unwrap()).collect::<Vec<_>>();
                            quote!{
                                #ident::#variant_ident { #(#items),* } => #input_ident::#variant_ident { #(#items: Input::from_normal(#items)),* }
                            }
                        }
                    }
                }).collect::<Vec<_>>();

                tokens.extend(quote! {
                    #[derive(Clone, Deserialize, Debug, PartialEq)] // TODO
                    pub enum #input_ident #ty #wher {
                        #(#input_variants),*
                    }
                });
                tokens.extend(quote! {
                    #[automatically_derived]
                    impl #imp Input for #input_ident #ty #wher {
                        type Normal = #ident #ty;
                        fn to_normal(&self) -> <Self as Input>::Normal {
                            match self {
                                #(#to_normal_variants),*
                            }
                        }
                        fn from_normal(normal: <Self as Input>::Normal) -> Self {
                            match normal {
                                #(#from_normal_variants),*
                            }
                        }
                        fn find_missing(&self) -> InputCheckResult {
                            let mut result = InputCheckResult::empty();
                            /*#(
                                let mut previous_result = self.#field_names.find_missing();
                                previous_result.extend_path(stringify!(#field_names).to_string());
                                result.union(&previous_result);
                            )**/
                            result
                        }
                        fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value){
                            //#(self.#field_names.insert_template_value(key, val);)*
                        }
                    }
                });
            }
            ast::Data::Struct(fields) => {
                match fields.style {
                    ast::Style::Struct => {
                        let field_names = fields
                            .iter()
                            .map(|f| f.ident.as_ref().map(|v| quote!(#v)).unwrap())
                            .collect::<Vec<_>>();

                        let input_type_tys = fields
                            .iter()
                            .enumerate()
                            .map(|(_i, f)| {
                                match &f.ty {
                                    syn::Type::Path(p) => {
                                        let ident_opt = p.path.get_ident();
                                        if let Some(ident) = ident_opt {
                                            ident.to_owned()
                                        } else {
                                            panic!(
                                                "{:?} is not a valid type for an Input struct.",
                                                p
                                            )
                                        }
                                    }
                                    _ => panic!(
                                        "{:?} is not a valid type for an Input struct.",
                                        f.ty
                                    ),
                                }
                                //f.ty.clone()
                            })
                            .collect::<Vec<_>>();

                        tokens.extend(quote! {
                                #[derive(Clone, Deserialize, Debug, PartialEq)] // TODO
                                pub struct #input_ident #ty #wher {
                                    #(pub #field_names: Value<<#input_type_tys as InputInverse>::Input>),*
                                }
                            });
                        tokens.extend(quote! {
                                #[automatically_derived]
                                impl #imp Input for #input_ident #ty #wher {
                                    type Normal = #ident #ty;
                                    fn to_normal(&self) -> <Self as Input>::Normal {
                                        Self::Normal {
                                            #(#field_names: self.#field_names.to_normal()),*
                                        }
                                    }
                                    fn from_normal(normal: <Self as Input>::Normal) -> Self {
                                        Self {
                                            #(#field_names: Value::Normal(Input::from_normal(normal.#field_names))),*
                                        }
                                    }
                                    fn find_missing(&self) -> InputCheckResult {
                                        let mut result = InputCheckResult::empty();
                                        #(
                                            let mut previous_result = self.#field_names.find_missing();
                                            previous_result.extend_path(stringify!(#field_names).to_string());
                                            result.union(&previous_result);
                                        )*
                                        result
                                    }
                                    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value){
                                        #(self.#field_names.insert_template_value(key, val);)*
                                    }
                                }
                            });
                    }
                    ast::Style::Tuple => {
                        // Generate the actual values to fill the format string.
                        let field_indexes = fields
                            .iter()
                            .enumerate()
                            .map(|(i, _f)| {
                                let i = syn::Index::from(i);
                                quote!(#i)
                            })
                            .collect::<Vec<_>>();

                        let input_type_tys = fields
                            .iter()
                            .enumerate()
                            .map(|(_i, f)| {
                                // This works with named or indexed fields, so we'll fall back to the index so we can
                                // write the output as a key-value pair.
                                match &f.ty {
                                    syn::Type::Path(p) => {
                                        let ident_opt = p.path.get_ident();
                                        if let Some(ident) = ident_opt {
                                            ident.to_owned()
                                        } else {
                                            panic!(
                                                "{:?} is not a valid type for an Input struct.",
                                                p
                                            )
                                        }
                                    }
                                    _ => panic!(
                                        "{:?} is not a valid type for an Input struct.",
                                        f.ty
                                    ),
                                }
                                //f.ty.clone()
                            })
                            .collect::<Vec<_>>();
                        tokens.extend(quote! {
                                #[derive(Clone, Deserialize, Debug, PartialEq)] // TODO
                                pub struct #input_ident #ty(#(Value<<#input_type_tys as InputInverse>::Input>),*) #wher;
                            });
                        tokens.extend(quote! {
                                #[automatically_derived]
                                impl #imp Input for #input_ident #ty #wher {
                                    type Normal = #ident #ty;
                                    fn to_normal(&self) -> <Self as Input>::Normal {
                                        #ident(#(self.#field_indexes.to_normal()),*)
                                    }
                                    fn from_normal(normal: <Self as Input>::Normal) -> Self {
                                        Self(#(Value::Normal(Input::from_normal(normal.#field_indexes))),*)
                                    }
                                    fn find_missing(&self) -> InputCheckResult {
                                        let mut result = InputCheckResult::empty();
                                        #(
                                            let mut previous_result = self.#field_indexes.find_missing();
                                            previous_result.extend_path(stringify!(#field_indexes).to_string());
                                            result.union(&previous_result);
                                        )*
                                        result
                                    }
                                    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value){
                                        #(self.#field_indexes.insert_template_value(key, val);)*
                                    }
                                }
                        });
                    }
                    ast::Style::Unit => {
                        tokens.extend(quote! {
                            #[derive(Clone, Deserialize, Debug, PartialEq)] // TODO
                            pub struct #input_ident #ty #wher;
                        });
                        tokens.extend(quote! {
                                #[automatically_derived]
                                impl #imp Input for #input_ident #ty #wher {
                                    type Normal = #ident #ty;
                                    fn to_normal(&self) -> <Self as Input>::Normal {
                                        #ident
                                    }
                                    fn from_normal(normal: <Self as Input>::Normal) -> Self {
                                        Self
                                    }
                                    fn find_missing(&self) -> InputCheckResult {
                                        InputCheckResult::empty()
                                    }
                                    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value){
                                    }
                                }
                        });
                    }
                }
            }
        }
        tokens.extend(quote! {
            #[automatically_derived]
            impl #imp InputInverse for #ident #ty #wher {
                type Input = #input_ident #ty;
            }
        });
    }
}

#[proc_macro_derive(Overwrite)]
pub fn derive_overwrite(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);

    let machine = match OverwriteReceiver::from_derive_input(&derive_input) {
        Ok(sm) => sm,
        Err(e) => panic!("error in derive(Overwrite): {}", e),
    };

    quote!(#machine).into()
}

#[derive(Debug, FromField)]
struct OverwriteFieldReceiver {
    /// Get the ident of the field. For fields in tuple or newtype structs or
    /// enum bodies, this can be `None`.
    ident: Option<syn::Ident>,

    /// This magic field name pulls the type from the input.
    ty: syn::Type,
}

#[derive(FromDeriveInput)]
#[darling(attributes(input))]
#[darling(supports(struct_any), forward_attrs(doc, derive))]
struct OverwriteReceiver {
    /// The struct ident.
    ident: syn::Ident,

    /// The type's generics. You'll need these any time your trait is expected
    /// to work with types that declare generics.
    generics: syn::Generics,

    /// Receives the body of the struct or enum. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    data: ast::Data<(), InputFieldReceiver>,
    attrs: Vec<syn::Attribute>,

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
        } = *self;

        /*let derive_attrs = attrs
            .iter()
            .filter(|a| a.path.is_ident("derive"))
            .collect::<Vec<_>>();
        eprintln!("{:?}", derive_attrs);*/

        let (imp, ty, wher) = generics.split_for_impl();
        let fields = data
            .as_ref()
            .take_struct()
            .expect("Should never be enum")
            .fields;

        // Generate the actual values to fill the format string.
        let field_names = fields
            .iter()
            .enumerate()
            .map(|(i, f)| {
                // This works with named or indexed fields, so we'll fall back to the index so we can
                // write the output as a key-value pair.
                let field_ident = f.ident.as_ref().map(|v| quote!(#v)).unwrap_or_else(|| {
                    let i = syn::Index::from(i);
                    quote!(#i)
                });
                field_ident
            })
            .collect::<Vec<_>>();

        let input_type_tys = fields
            .into_iter()
            .enumerate()
            .map(|(_i, f)| {
                // This works with named or indexed fields, so we'll fall back to the index so we can
                // write the output as a key-value pair.
                match &f.ty {
                    syn::Type::Path(p) => {
                        let ident_opt = p.path.get_ident();
                        if let Some(ident) = ident_opt {
                            ident.to_owned()
                        } else {
                            panic!("{:?} is not a valid type for an Input struct.", p)
                        }
                    }
                    _ => panic!("{:?} is not a valid type for an Input struct.", f.ty),
                }
                //f.ty.clone()
            })
            .collect::<Vec<_>>();
        let input_ident = syn::Ident::new(&input_name, ident.span());

        tokens.extend(quote! {
            #[automatically_derived]
            impl #imp Overwrite<#input_ident #ty> for #input_ident #ty #wher {
                fn overwrite(&mut self, other: &Self){
                    #(self.#field_names.overwrite(&other.#field_names);)*
                }
            }
        });
    }
}
