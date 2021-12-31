use darling::ast;
use darling::{FromDeriveInput, FromField, FromVariant};
use quote::{quote, ToTokens};

#[derive(Debug, FromField)]
#[darling(forward_attrs)]
pub struct InputFieldReceiver {
    /// Get the ident of the field. For fields in tuple or newtype structs or
    /// enum bodies, this can be `None`.
    pub ident: Option<syn::Ident>,

    pub ty: syn::Type,
    pub attrs: Vec<syn::Attribute>,
}

#[derive(Debug, FromVariant)]
#[darling(forward_attrs)]
pub struct InputVariantReceiver {
    pub ident: syn::Ident,
    pub fields: ast::Fields<InputFieldReceiver>,
    pub attrs: Vec<syn::Attribute>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(input))]
#[darling(forward_attrs)]
pub struct InputReceiver {
    ident: syn::Ident,

    /// The type's generics. You'll need these any time your trait is expected
    /// to work with types that declare generics.
    generics: syn::Generics,

    data: ast::Data<InputVariantReceiver, InputFieldReceiver>,
    attrs: Vec<syn::Attribute>,

    #[darling(rename = "name")]
    input_name: String,

    #[darling(default)]
    test: bool,
    #[darling(default)]
    no_examples: bool,
}

fn get_input_type(t: &syn::Type) -> proc_macro2::TokenStream {
    quote!(#t)
}

pub fn get_input_types(fields: &[InputFieldReceiver]) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .enumerate()
        .map(|(_i, f)| get_input_type(&f.ty))
        .collect::<Vec<_>>()
}

// First result is for the main type (Input or enum), the second for the Input when there is a
// enum version
fn handle_attributes(
    attrs: &[syn::Attribute],
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let derive_attrs = attrs
        .iter()
        .filter(|a| a.path.is_ident("derive"))
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

    let mut tokens_main = quote!(#[derive(#(#derive_attrs),*)]);
    let mut tokens_second = quote!(#[derive(#(#derive_attrs),*)]);
    let other_attrs = attrs
        .iter()
        .filter(|a| !a.path.is_ident("derive"))
        .map(|a| quote! {#a})
        .collect::<Vec<_>>();
    tokens_main.extend(quote!(#(#other_attrs)*));

    let other_attrs = attrs
        .iter()
        .filter(|a| !a.path.is_ident("derive") && !is_serde_from(a))
        .map(|a| quote! {#a})
        .collect::<Vec<_>>();
    tokens_second.extend(quote!(#(#other_attrs)*));

    (tokens_main, tokens_second)
}

fn is_serde_from(a: &syn::Attribute) -> bool {
    if a.path.is_ident("serde") {
        match a.parse_meta() {
            Ok(syn::Meta::List(l)) => l.nested.into_iter().any(|a| match a {
                syn::NestedMeta::Meta(syn::Meta::NameValue(meta)) => {
                    meta.path.is_ident("try_from") || meta.path.is_ident("from")
                }
                _ => false,
            }),
            _ => false,
        }
    } else {
        false
    }
}

fn input_handle_unit_struct(
    ident: &syn::Ident,
    input_ident: &syn::Ident,
    generics: &syn::Generics,
    input_attributes: &proc_macro2::TokenStream,
    tokens: &mut proc_macro2::TokenStream,
) {
    let (imp, ty, wher) = generics.split_for_impl();
    tokens.extend(quote! {
        #input_attributes
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
                fn files_to_load(&self) -> Vec<FileToLoad> {
                    Vec::new()
                }
                fn insert_loaded_files(&mut self, _files: &std::collections::HashMap<FileToLoad, LoadedFile>) {
                }
            }
    });
    // Also implement InputInverse
    tokens.extend(quote! {
        #[automatically_derived]
        impl #imp InputInverse for #ident #ty #wher {
            type Input = #input_ident #ty;
            type EnumInput = Self::Input;
        }
    });
}

fn input_handle_tuple_struct(
    fields: &ast::Fields<InputFieldReceiver>,
    ident: &syn::Ident,
    input_ident: &syn::Ident,
    generics: &syn::Generics,
    input_attributes: &proc_macro2::TokenStream,
    tokens: &mut proc_macro2::TokenStream,
) {
    let (imp, ty, wher) = generics.split_for_impl();
    let input_type_tys = get_input_types(&fields.fields);
    let field_indexes = fields
        .iter()
        .enumerate()
        .map(|(i, _f)| {
            let i = syn::Index::from(i);
            quote!(#i)
        })
        .collect::<Vec<_>>();

    tokens.extend(quote! {
        #input_attributes
        pub struct #input_ident #ty(#(ValueType<<#input_type_tys as InputInverse>::Input>),*) #wher;
    });
    tokens.extend(quote! {
            #[automatically_derived]
            impl #imp Input for #input_ident #ty #wher {
                type Normal = #ident #ty;
                fn to_normal(&self) -> <Self as Input>::Normal {
                    #ident(#(self.#field_indexes.to_normal()),*)
                }
                fn from_normal(normal: <Self as Input>::Normal) -> Self {
                    Self(#(ValueType::Normal(Input::from_normal(normal.#field_indexes))),*)
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
                fn files_to_load(&self) -> Vec<FileToLoad> {
                    let mut result = Vec::new();
                    #(
                        let previous_result = self.#field_indexes.files_to_load();
                        result.extend(previous_result);
                    )*
                    result
                }
                fn insert_loaded_files(&mut self, files: &std::collections::HashMap<FileToLoad, LoadedFile>) {
                    #(self.#field_indexes.insert_loaded_files(files);)*
                }
            }
    });
    // Also implement InputInverse
    tokens.extend(quote! {
        #[automatically_derived]
        impl #imp InputInverse for #ident #ty #wher {
            type Input = #input_ident #ty;
            type EnumInput = Self::Input;
        }
    });
}

fn input_handle_struct_struct(
    fields: &ast::Fields<InputFieldReceiver>,
    ident: &syn::Ident,
    input_ident: &syn::Ident,
    generics: &syn::Generics,
    input_attributes: &(proc_macro2::TokenStream, proc_macro2::TokenStream),
    tokens: &mut proc_macro2::TokenStream,
) {
    // Beginning of fixing generics
    /*let mut new_generics = generics.clone();
    if new_generics.params.len() > 0 {
        let new_where = new_generics.make_where_clause();
        new_where
            .predicates
            .push(parse_quote! {T : Input + InputInverse});
    }
    let (input_imp, input_ty, input_wher) = new_generics.split_for_impl();
        */

    let (imp, ty, wher) = generics.split_for_impl();
    let input_type_tys = get_input_types(&fields.fields);
    let field_names = fields
        .iter()
        .map(|f| f.ident.as_ref().map(|v| quote!(#v)).unwrap())
        .collect::<Vec<_>>();

    let field_attributes = fields
        .iter()
        .map(|f| {
            f.attrs
                .iter()
                .filter(|a| !a.path.is_ident("input"))
                .map(|a| quote!(#a))
                .collect::<Vec<_>>()
        })
        .map(|a| quote!(#(#a)*))
        .collect::<Vec<_>>();

    let field_is_flattened = fields
        .iter()
        .map(|f| {
            f.attrs.iter().any(|attr| {
                if attr.path.is_ident("serde") {
                    match attr.parse_meta() {
                        Ok(syn::Meta::List(meta)) => meta.nested.iter().any(|m| match m {
                            syn::NestedMeta::Meta(syn::Meta::Path(m)) => m.is_ident("flatten"),
                            _ => false,
                        }),
                        _ => false,
                    }
                } else {
                    false
                }
            })
        })
        .collect::<Vec<_>>();

    let enum_input_ident = syn::Ident::new(
        &format!("{}Enum", input_ident.to_string())[..],
        input_ident.span(),
    );

    let input_attributes_input = &input_attributes.0;
    let input_attributes_enum = &input_attributes.1;

    let field_types = input_type_tys
        .iter()
        .zip(field_is_flattened.iter())
        .map(|(t, flattened)| {
            if *flattened {
                quote!(<#t as InputInverse>::Input)
            } else {
                quote!(Value<<#t as InputInverse>::Input>)
            }
        })
        .collect::<Vec<_>>();

    tokens.extend(quote! {
        #input_attributes_input
        pub struct #input_ident #ty #wher {
            #(
                #field_attributes
                pub #field_names: #field_types
            ),*
        }
    });

    let try_from_value = input_ident.to_string();

    let try_from = quote! {#[serde(try_from = #try_from_value)]};

    tokens.extend(quote! {
        #input_attributes_enum
        #try_from
        pub struct #enum_input_ident #ty (pub #input_ident) #wher;
    });
    let from_normal_lines = field_names
        .iter()
        .zip(field_is_flattened.iter())
        .map(|(f, flattened)| {
            if *flattened {
                quote!(Input::from_normal(normal.#f))
            } else {
                quote!(Value::Normal(Input::from_normal(normal.#f)))
            }
        })
        .collect::<Vec<_>>();

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
                    #(#field_names: #from_normal_lines),*
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
            fn files_to_load(&self) -> Vec<FileToLoad> {
                let mut result = Vec::new();
                #(
                    let previous_result = self.#field_names.files_to_load();
                    result.extend(previous_result);
                )*
                result
            }

            fn insert_loaded_files(&mut self, files: &std::collections::HashMap<FileToLoad, LoadedFile>) {
                #(self.#field_names.insert_loaded_files(files);)*
            }
        }
    });

    tokens.extend(quote! {
        #[automatically_derived]
        impl #imp Input for #enum_input_ident #ty #wher {
            type Normal = #ident #ty;
            fn to_normal(&self) -> <Self as Input>::Normal {
                self.0.to_normal()
            }
            fn from_normal(normal: <Self as Input>::Normal) -> Self {
                Self(<#input_ident>::from_normal(normal))
            }
            fn find_missing(&self) -> InputCheckResult {
                self.0.find_missing()
            }
            fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value){
                self.0.insert_template_value(key, val)
            }
            fn files_to_load(&self) -> Vec<FileToLoad> {
                self.0.files_to_load()
            }

            fn insert_loaded_files(&mut self, files: &std::collections::HashMap<FileToLoad, LoadedFile>) {
                self.0.insert_loaded_files(files);
            }
        }
    });

    let try_from_code = if field_is_flattened.iter().any(|a| *a) {
        quote! {Ok(Self(value))}
    } else {
        quote! {
            let mut ok = false;
            #(
                if value.#field_names.is_some() {
                    ok = true;
                }
            )*
            if ok {
                Ok(Self(value))
            } else {
                Err("No field is set to a not-none value.")
            }

        }
    };

    tokens.extend(quote! {
        #[automatically_derived]
        impl std::convert::TryFrom<#input_ident #ty> for #enum_input_ident #ty #wher {
            type Error = &'static str;

            fn try_from(value: #input_ident) -> Result<Self, Self::Error> {
                #try_from_code
            }
        }
    });
    tokens.extend(quote! {
        #[automatically_derived]
        impl std::convert::From<#enum_input_ident #ty> for #input_ident #ty #wher {
            fn from(value: #enum_input_ident) -> Self {
                value.0
            }
        }
    });
    // Also implement InputInverse
    tokens.extend(quote! {
        #[automatically_derived]
        impl #imp InputInverse for #ident #ty #wher {
            type Input = #input_ident #ty;
            type EnumInput = #enum_input_ident #ty;
        }
    });
}

fn input_handle_struct(
    fields: &ast::Fields<InputFieldReceiver>,
    ident: &syn::Ident,
    input_ident: &syn::Ident,
    generics: &syn::Generics,
    input_attributes: &(proc_macro2::TokenStream, proc_macro2::TokenStream),
    tokens: &mut proc_macro2::TokenStream,
) {
    match fields.style {
        ast::Style::Struct => input_handle_struct_struct(
            fields,
            ident,
            input_ident,
            generics,
            input_attributes,
            tokens,
        ),
        ast::Style::Tuple => input_handle_tuple_struct(
            fields,
            ident,
            input_ident,
            generics,
            &input_attributes.0,
            tokens,
        ),
        ast::Style::Unit => {
            input_handle_unit_struct(ident, input_ident, generics, &input_attributes.0, tokens)
        }
    }
}

// Create all enum variants for the input enum
fn input_handle_enum_input_variants(v: &[InputVariantReceiver]) -> Vec<proc_macro2::TokenStream> {
    v.iter()
        .map(|variant| {
            let input_type_tys = get_input_types(&variant.fields.fields);
            let variant_ident = &variant.ident;

            let variant_attributes = variant.attrs.iter().map(|a| quote!(#a)).collect::<Vec<_>>();
            match variant.fields.style {
                ast::Style::Unit => {
                    quote! {
                        #(#variant_attributes)*
                        #variant_ident
                    }
                }
                ast::Style::Tuple => {
                    let items = quote!(#(<#input_type_tys as InputInverse>::EnumInput),*);

                    quote! {
                        #(#variant_attributes)*
                        #variant_ident(#items)
                    }
                }
                ast::Style::Struct => {
                    let items = variant
                        .fields
                        .fields
                        .iter()
                        .map(|f| f.ident.as_ref().map(|a| quote!(#a)).unwrap())
                        .collect::<Vec<_>>();

                    let field_attributes = variant
                        .fields
                        .fields
                        .iter()
                        .map(|f| f.attrs.iter().map(|a| quote!(#a)).collect::<Vec<_>>())
                        .map(|a| quote!(#(#a)*))
                        .collect::<Vec<_>>();
                    quote! {
                        #(#variant_attributes)*
                        #variant_ident {
                            #(
                            #field_attributes
                            #items: Value<<#input_type_tys as InputInverse>::Input>
                            ),*
                        }
                    }
                }
            }
        })
        .collect::<Vec<_>>()
}

// Create tokens for each enum variant for the to_normal method
fn input_handle_enum_to_normal_variants(
    v: &[InputVariantReceiver],
    ident: &syn::Ident,
    input_ident: &syn::Ident,
) -> Vec<proc_macro2::TokenStream> {
    v.iter().map(|variant| {
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
    }).collect::<Vec<_>>()
}

// Create tokens for each enum variant for the from_normal method
fn input_handle_enum_from_normal_variants(
    v: &[InputVariantReceiver],
    ident: &syn::Ident,
    input_ident: &syn::Ident,
) -> Vec<proc_macro2::TokenStream> {
    v.iter().map(|variant| {
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
    }).collect::<Vec<_>>()
}

// Create tokens for each enum variant for the find_missing method
fn input_handle_enum_find_missing_variants(
    v: &[InputVariantReceiver],
    input_ident: &syn::Ident,
) -> Vec<proc_macro2::TokenStream> {
    v.iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            match variant.fields.style {
                ast::Style::Unit => {
                    quote! {
                        #input_ident::#variant_ident => InputCheckResult::empty()
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
                    quote! {
                        #input_ident::#variant_ident(#(#items),*) => {
                            let mut result = InputCheckResult::empty();
                            #(
                                let mut previous_result = #items.find_missing();
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
                        #input_ident::#variant_ident { #(#items),* } => {
                            let mut result = InputCheckResult::empty();
                            #(
                                let mut previous_result = #items.find_missing();
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

// Create tokens for each enum variant for the insert_template_value_variants method
fn input_handle_enum_insert_template_value_variants(
    v: &[InputVariantReceiver],
    input_ident: &syn::Ident,
) -> Vec<proc_macro2::TokenStream> {
    v.iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            match variant.fields.style {
                ast::Style::Unit => {
                    quote! {
                        #input_ident::#variant_ident => ()
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
                    quote! {
                        #input_ident::#variant_ident(#(#items),*) => {
                            #(#items.insert_template_value(key, val));*
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
                        #input_ident::#variant_ident { #(#items),* } => {
                            #(#items.insert_template_value(key, val));*
                        }
                    }
                }
            }
        })
        .collect::<Vec<_>>()
}

// Create tokens for each enum variant for the find_missing method
fn input_handle_enum_files_to_load_variants(
    v: &[InputVariantReceiver],
    input_ident: &syn::Ident,
) -> Vec<proc_macro2::TokenStream> {
    v.iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            match variant.fields.style {
                ast::Style::Unit => {
                    quote! {
                        #input_ident::#variant_ident => Vec::new()
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
                    quote! {
                        #input_ident::#variant_ident(#(#items),*) => {
                            let mut result = Vec::new();
                            #(
                                let previous_result = #items.files_to_load();
                                result.extend(previous_result);
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
                        #input_ident::#variant_ident { #(#items),* } => {
                            let mut result = Vec::new();
                            #(
                                let previous_result = #items.files_to_load();
                                result.extend(previous_result);
                            )*
                            result
                        }
                    }
                }
            }
        })
        .collect::<Vec<_>>()
}

// Create tokens for each enum variant for the insert_template_value_variants method
fn input_handle_enum_insert_loaded_files_variants(
    v: &[InputVariantReceiver],
    input_ident: &syn::Ident,
) -> Vec<proc_macro2::TokenStream> {
    v.iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            match variant.fields.style {
                ast::Style::Unit => {
                    quote! {
                        #input_ident::#variant_ident => ()
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
                    quote! {
                        #input_ident::#variant_ident(#(#items),*) => {
                            #(#items.insert_loaded_files(files);)*
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
                        #input_ident::#variant_ident { #(#items),* } => {
                            #(#items.insert_loaded_files(files));*
                        }
                    }
                }
            }
        })
        .collect::<Vec<_>>()
}

// Handle derivation for enums
fn input_handle_enum(
    v: &[InputVariantReceiver],
    ident: &syn::Ident,
    input_ident: &syn::Ident,
    generics: &syn::Generics,
    input_attributes: &proc_macro2::TokenStream,
    tokens: &mut proc_macro2::TokenStream,
) {
    let (imp, ty, wher) = generics.split_for_impl();

    let input_variants = input_handle_enum_input_variants(v);

    let to_normal_variants = input_handle_enum_to_normal_variants(v, ident, input_ident);

    let from_normal_variants = input_handle_enum_from_normal_variants(v, ident, input_ident);

    let find_missing_variants = input_handle_enum_find_missing_variants(v, input_ident);

    let files_to_load_variants = input_handle_enum_files_to_load_variants(v, input_ident);

    let insert_loaded_files_variants =
        input_handle_enum_insert_loaded_files_variants(v, input_ident);

    let insert_template_value_variants =
        input_handle_enum_insert_template_value_variants(v, input_ident);

    tokens.extend(quote! {
        #input_attributes
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
                match self {
                    #(#find_missing_variants),*
                }
            }
            fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value){
                match self {
                    #(#insert_template_value_variants),*
                }
            }
            fn files_to_load(&self) -> Vec<FileToLoad> {
                match self {
                    #(#files_to_load_variants),*
                }
            }

            fn insert_loaded_files(&mut self, files: &std::collections::HashMap<FileToLoad, LoadedFile>) {
                match self {
                    #(#insert_loaded_files_variants),*
                }
            }
        }
    });
    // Also implement InputInverse
    tokens.extend(quote! {
        #[automatically_derived]
        impl #imp InputInverse for #ident #ty #wher {
            type Input = #input_ident #ty;
            type EnumInput = Self::Input;
        }
    });
}

impl ToTokens for InputReceiver {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let InputReceiver {
            ref ident,
            ref generics,
            ref data,
            ref attrs,
            ref input_name,
            test: _,
            no_examples: _,
        } = *self;

        let input_ident = syn::Ident::new(input_name, ident.span());
        let input_attributes = handle_attributes(attrs);

        match data {
            ast::Data::Enum(v) => input_handle_enum(
                v,
                ident,
                &input_ident,
                generics,
                &input_attributes.0,
                tokens,
            ),
            ast::Data::Struct(fields) => input_handle_struct(
                fields,
                ident,
                &input_ident,
                generics,
                &input_attributes,
                tokens,
            ),
        }
    }
}
