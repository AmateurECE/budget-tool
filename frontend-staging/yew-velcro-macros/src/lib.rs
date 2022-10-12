///////////////////////////////////////////////////////////////////////////////
// NAME:            lib.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoints containing derive macros for use with the
//                  yew-velcro crate
//
// CREATED:         10/12/2022
//
// LAST EDITED:     10/12/2022
////

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Field, Fields,
    GenericParam, Generics, Lit, Meta, NestedMeta,
};

///////////////////////////////////////////////////////////////////////////////
// Fields
////

#[proc_macro_derive(Fields)]
pub fn derive_fields(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // This allows us to add a bound "T: ToString" for every type parameter "T"
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let field_creation = create_fields(&input.data);
    let expanded = quote! {
        // The generated impl
        impl #impl_generics ::yew_velcro::Fields for #name #ty_generics
            #where_clause
        {
            fn fields(&self) -> FieldView {
                #field_creation
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

// Add a bound `T: ToString` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(::std::string::ToString));
        }
    }
    generics
}

fn create_fields(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                // Expands to an expression like
                //      vec![self.x.to_string(), self.y.to_string()].into()
                // We take some care to use the span of each `syn::Field` as
                // the span of the corresponding `to_string()` call. This way
                // if one of the field types does not implement `ToString` then
                // the compiler's error message underlines which field it is.
                let field_views = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! {
                        f.span() =>
                            ::std::string::ToString::to_string(&self.#name)
                    }
                });
                quote! {
                    vec![ #(#field_views ,)* ].into()
                }
            }

            Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
        },

        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}

///////////////////////////////////////////////////////////////////////////////
// Fields
////

#[proc_macro_derive(FieldNames, attributes(field_name))]
pub fn derive_field_names(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) =
        input.generics.split_for_impl();

    let field_creation = create_field_names(&input.data);
    let expanded = quote! {
        // The generated impl
        impl #impl_generics ::yew_velcro::FieldNames for #name #ty_generics
            #where_clause
        {
            fn field_names() -> FieldSpec {
                #field_creation
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

// Create the expression that generates a FieldView for the impl
fn create_field_names(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                // Expands to an expression like
                //      vec!["x".to_string(), "y".to_string()].into()
                let field_names = fields.named.iter().map(|f| {
                    let name = get_field_name(f)
                        .unwrap_or_else(|| "".to_string());
                    quote_spanned! {
                        f.span() =>
                            ::std::string::ToString::to_string(#name)
                    }
                });
                quote! {
                    vec![ #(#field_names ,)* ].into()
                }
            }

            Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
        },

        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}

// Obtain the field name, potentially parsing any present attributes
fn get_field_name(field: &Field) -> Option<String> {
    field
        .attrs
        .iter()
        .find(|a| a.path.is_ident("field_name"))
        .and_then(|a| {
            let meta = a
                .parse_meta()
                .expect("Expected an attribute of the form 'name = value'");
            if let Meta::List(ref list) = meta {
                list.nested
                    .iter()
                    .filter_map(|nested| {
                        if let NestedMeta::Meta(ref meta) = nested {
                            if let Meta::NameValue(ref value) = meta {
                                assert!(meta.path().is_ident("rename"));
                                if let Lit::Str(string) = &value.lit {
                                    Some(string.value())
                                } else {
                                    None
                                }
                            } else {
                                panic!(
                                    "\"field_name\" attribute requires a list \
                                    of key/value pairs"
                                )
                            }
                        } else {
                            panic!(
                                "\"field_name\" attribute requires a list of \
                                key/value pairs"
                            )
                        }
                    })
                    .last()
            } else {
                panic!(
                    "\"field_name\" attribute requires a list of key/value \
                        pairs"
                )
            }
        })
        .or_else(|| field.ident.as_ref().map(|i| i.to_string()))
}

///////////////////////////////////////////////////////////////////////////////
