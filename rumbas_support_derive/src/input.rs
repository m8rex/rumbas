use darling::ast;
use darling::{FromDeriveInput, FromField, FromVariant};
use quote::{quote, ToTokens};

#[derive(Debug, FromField)]
#[darling(forward_attrs)]
pub struct InputFieldReceiver {
    /// Get the ident of the field. For fields in tuple or newtype structs or
    /// enum bodies, this can be `None`.
    pub ident: Option<syn::Ident>,

    /// This magic field name pulls the type from the input.
    pub ty: syn::Type,
    pub attrs: Vec<syn::Attribute>,
}

#[derive(Debug, FromVariant)]
#[darling(forward_attrs)]
pub struct InputVariantReceiver {
    /// The identifier of the passed-in variant
    pub ident: syn::Ident,
    // The fields associated with the variant
    pub fields: ast::Fields<InputFieldReceiver>,
    pub attrs: Vec<syn::Attribute>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(input))]
#[darling(forward_attrs)]
pub struct InputReceiver {
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

fn get_input_type(t: &syn::Type) -> proc_macro2::TokenStream {
    match t {
        syn::Type::Path(p) => {
            quote!(#p)
        }
        syn::Type::Group(g) => get_input_type(&*g.elem),
        _ => panic!("{:?} is not a valid type for an Input struct.", t), // Needed?
    }
}

pub fn get_input_types(fields: &Vec<InputFieldReceiver>) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .enumerate()
        .map(|(_i, f)| get_input_type(&f.ty))
        .collect::<Vec<_>>()
}

fn handle_attributes(attrs: &Vec<syn::Attribute>) -> proc_macro2::TokenStream {
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
    //println!("{:#?}", derive_attrs);
    let mut tokens = quote!(#[derive(#(#derive_attrs),*)]);
    let other_attrs = attrs
        .iter()
        .filter(|a| !a.path.is_ident("derive"))
        .map(|a| quote! {#a})
        .collect::<Vec<_>>();
    tokens.extend(quote!(#(#other_attrs)*));
    //println!("{:#?}", tokens);
    tokens
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
            }
    });
}

fn input_handle_struct_struct(
    fields: &ast::Fields<InputFieldReceiver>,
    ident: &syn::Ident,
    input_ident: &syn::Ident,
    generics: &syn::Generics,
    input_attributes: &proc_macro2::TokenStream,
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
        .map(|f| f.attrs.iter().map(|a| quote!(#a)).collect::<Vec<_>>())
        .map(|a| quote!(#(#a)*))
        .collect::<Vec<_>>();

    tokens.extend(quote! {
        #input_attributes
        pub struct #input_ident #ty #wher {
            #(
                #field_attributes
                pub #field_names: Value<<#input_type_tys as InputInverse>::Input>
            ),*
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

fn input_handle_struct(
    fields: &ast::Fields<InputFieldReceiver>,
    ident: &syn::Ident,
    input_ident: &syn::Ident,
    generics: &syn::Generics,
    input_attributes: &proc_macro2::TokenStream,
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
            input_attributes,
            tokens,
        ),
        ast::Style::Unit => {
            input_handle_unit_struct(ident, input_ident, generics, input_attributes, tokens)
        }
    }
}

// Create all enum variants for the input enum
fn input_handle_enum_input_variants(
    v: &Vec<InputVariantReceiver>,
) -> Vec<proc_macro2::TokenStream> {
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
                    let items = quote!(#(<#input_type_tys as InputInverse>::Input),*);

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

//
fn input_handle_enum_to_normal_variants(
    v: &Vec<InputVariantReceiver>,
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

fn input_handle_enum_from_normal_variants(
    v: &Vec<InputVariantReceiver>,
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

fn input_handle_enum_find_missing_variants(
    v: &Vec<InputVariantReceiver>,
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
                        #input_ident::#variant_ident(#(#items),*) => {
                            let mut result = InputCheckResult::empty();
                            #(
                                let mut previous_result = #items.find_missing();
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

fn input_handle_enum_insert_template_value_variants(
    v: &Vec<InputVariantReceiver>,
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

fn input_handle_enum(
    v: &Vec<InputVariantReceiver>,
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
        } = *self;

        //println!("{:?}", attrs);

        /*let derive_attrs = attrs
            .iter()
            .filter(|a| a.path.is_ident("derive"))
            .collect::<Vec<_>>();
        eprintln!("{:?}", derive_attrs);*/

        let (imp, ty, wher) = generics.split_for_impl();

        let input_ident = syn::Ident::new(&input_name, ident.span());
        let input_attributes = handle_attributes(&attrs);

        match data {
            ast::Data::Enum(v) => {
                input_handle_enum(v, ident, &input_ident, generics, &input_attributes, tokens)
            }
            ast::Data::Struct(fields) => input_handle_struct(
                fields,
                ident,
                &input_ident,
                generics,
                &input_attributes,
                tokens,
            ),
        }
        // Also implement InputInverse
        tokens.extend(quote! {
            #[automatically_derived]
            impl #imp InputInverse for #ident #ty #wher {
                type Input = #input_ident #ty;
            }
        });
    }
}
