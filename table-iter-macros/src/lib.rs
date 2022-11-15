///////////////////////////////////////////////////////////////////////////////
// NAME:            lib.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoints containing derive macros for use with the
//                  table-iter crate
//
// CREATED:         10/12/2022
//
// LAST EDITED:     11/15/2022
//
// Copyright 2022, Ethan D. Twardy
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
////

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Data, DeriveInput, Field, Fields, Lit, Meta, NestedMeta,
};

///////////////////////////////////////////////////////////////////////////////
// Attribute Helper
////

const NOT_LIST_ERROR: &'static str = "\
\"field_name\" attribute requires a list of key/value pairs";

// For field, search for an attribute of the #[fields] helper macro which
// matches the identifier `attribute'.
fn get_fields_attribute(field: &Field, attribute: &str) -> Option<Meta> {
    field
        .attrs
        .iter()
        .find(|a| a.path.is_ident("fields"))
        .and_then(|a| {
            let meta = a.parse_meta().expect(NOT_LIST_ERROR);
            if let Meta::List(ref list) = meta {
                list.nested.iter().find_map(|nested| match nested {
                    NestedMeta::Meta(ref meta) => {
                        if meta.path().is_ident(attribute) {
                            Some(meta.clone())
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
            } else {
                panic!("{}", NOT_LIST_ERROR)
            }
        })
}

fn is_skipped_field(field: &Field) -> bool {
    get_fields_attribute(field, "skip").is_some()
}

///////////////////////////////////////////////////////////////////////////////
// Fields
////

#[proc_macro_derive(Fields)]
pub fn derive_fields(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) =
        input.generics.split_for_impl();

    let field_creation = create_fields(&input.data);
    let expanded = quote! {
        // The generated impl
        impl #impl_generics ::table_iter::fields::Fields for #name #ty_generics
            #where_clause
        {
            fn fields(&self) -> ::table_iter::fields::FieldView {
                #field_creation
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
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
                let field_views =
                    fields.named.iter().filter(|f| !is_skipped_field(f)).map(
                        |f| {
                            let name = &f.ident;
                            let display_function = get_display_function(f)
                                .unwrap_or_else(|| {
                                    quote!(::std::string::ToString::to_string)
                                });
                            quote_spanned! {
                                f.span() =>
                                    #display_function(&self.#name)
                            }
                        },
                    );
                quote! {
                    vec![ #(#field_views ,)* ].into()
                }
            }

            Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
        },

        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn get_display_function(field: &Field) -> Option<TokenStream> {
    get_fields_attribute(field, "with").and_then(|meta| {
        if let Meta::NameValue(ref value) = meta {
            if let Lit::Str(literal) = &value.lit {
                Some(literal.parse().unwrap())
            } else {
                None
            }
        } else {
            panic!("rename attribute expects key/value pair")
        }
    })
}

///////////////////////////////////////////////////////////////////////////////
// Field Names
////

#[proc_macro_derive(FieldNames, attributes(fields))]
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
        impl #impl_generics ::table_iter::fields::FieldNames for #name
            #ty_generics #where_clause
        {
            fn field_names() -> ::table_iter::fields::FieldSpec {
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
                let field_names =
                    fields.named.iter().filter(|f| !is_skipped_field(f)).map(
                        |f| {
                            let name = get_field_name(f)
                                .unwrap_or_else(|| "".to_string());
                            quote_spanned! {
                                f.span() =>
                                    ::std::string::ToString::to_string(#name)
                            }
                        },
                    );
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
    get_fields_attribute(field, "rename")
        .and_then(|meta| {
            if let Meta::NameValue(ref value) = meta {
                if let Lit::Str(string) = &value.lit {
                    Some(string.value())
                } else {
                    None
                }
            } else {
                panic!("rename attribute expects key/value pair")
            }
        })
        .or_else(|| field.ident.as_ref().map(|i| i.to_string()))
}

///////////////////////////////////////////////////////////////////////////////
