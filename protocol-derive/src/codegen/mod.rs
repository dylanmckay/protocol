use proc_macro2::TokenStream;
use syn;
use syn::{Field};

use crate::attr;

pub mod enums;

pub fn read_struct_field(fields: &syn::Fields)
                         -> TokenStream {
    match *fields {
        syn::Fields::Named(ref fields_named) => read_named_fields_struct(fields_named),
        syn::Fields::Unnamed(ref fields_unnamed) => read_unnamed_fields(fields_unnamed),
        syn::Fields::Unit => quote!(),
    }
}

pub fn read_enum_fields(fields: &syn::Fields)
                        -> TokenStream {
    match *fields {
        syn::Fields::Named(ref fields_named) => read_named_fields_enum(fields_named),
        syn::Fields::Unnamed(ref fields_unnamed) => read_unnamed_fields(fields_unnamed),
        syn::Fields::Unit => quote!(),
    }
}

pub fn write_fields(fields: &syn::Fields)
                    -> TokenStream {
    match *fields {
        syn::Fields::Named(ref fields_named) => write_named_fields(fields_named),
        syn::Fields::Unnamed(ref fields_unnamed) => write_unnamed_fields(fields_unnamed),
        syn::Fields::Unit => quote!(),
    }
}

pub fn named_fields_declarations(fields: &syn::Fields) -> TokenStream {
    if let syn::Fields::Named(ref fields_named) = fields {
        let fields_variables: Vec<TokenStream> = fields_named.named.iter().map(|field| {
            let field_name = &field.ident;
            let field_ty = &field.ty;
            // This field may store the length prefix of one or more other field.
            let update_hints = update_hints_after_read(field, &fields_named.named);
            let update_hints_fixed = update_hint_fixed_length(field, &fields_named.named);

            if let Some(skip_condition) = maybe_skip(field.clone()) {
                quote! {
                    #update_hints_fixed
                    __hints.set_skip(#skip_condition);
                    let #field_name: #field_ty = protocol::Parcel::read_field(__io_reader, __settings, &mut __hints)?;
                    #update_hints
                    __hints.next_field();
                }
            } else {
                quote! {
                    #update_hints_fixed
                    let #field_name: #field_ty = protocol::Parcel::read_field(__io_reader, __settings, &mut __hints)?;
                    #update_hints
                    __hints.next_field();
                }
            }
        }).collect();

        quote! {
            #( #fields_variables)*
        }
    } else {
        quote!()
    }
}

/// Generates code that builds a initializes
/// an item with named fields by parsing
/// each of the fields.
///
/// Returns  `{ ..field initializers.. }`.
fn read_named_fields_enum(fields_named: &syn::FieldsNamed)
                          -> TokenStream {
    let field_initializers: Vec<_> = fields_named.named.iter().map(|field| {
        let field_name = &field.ident;
        let field_ty = &field.ty;
        // This field may store the length prefix of one or more other field.
        let update_hints = update_hints_after_read(field, &fields_named.named);
        let update_hints_fixed = update_hint_fixed_length(field, &fields_named.named);

        quote! {
            #field_name : {
                #update_hints_fixed
                let res: Result<#field_ty, _> = protocol::Parcel::read_field(__io_reader, __settings, &mut __hints);
                #update_hints
                __hints.next_field();
                res?
            }
        }
    }).collect();

    quote! { { #( #field_initializers ),* } }
}

/// Generates code that builds a initializes
/// an item with named fields by parsing
/// each of the fields.
///
/// Returns  `{ ..field initializers.. }`.
fn read_named_fields_struct(fields_named: &syn::FieldsNamed)
                            -> TokenStream {
    let field_initializers: Vec<_> = fields_named.named.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name }
    }).collect();

    quote! { { #( #field_initializers ),* } }
}

fn update_hints_after_read<'a>(field: &'a syn::Field,
                               fields: impl IntoIterator<Item=&'a syn::Field> + Clone)
                               -> TokenStream {
    let hint_setters = length_prefix_of(field, fields.clone()).iter().map(|(length_prefix_of, kind, prefix_subfield_names)| {
        let kind = kind.path_expr();
        let field_name = &field.ident;
        quote! {
            __hints.set_field_length(#length_prefix_of,
                (#field_name #(.#prefix_subfield_names)* ).clone() as usize,
                #kind);
        }
    }).collect::<Vec<TokenStream>>();

    if hint_setters.is_empty() {
        quote! {}
    } else {
        quote! {
                #(#hint_setters);*
        }
    }
}

fn update_hint_fixed_length<'a>(field: &'a syn::Field,
                                fields: impl IntoIterator<Item=&'a syn::Field> + Clone)
                                -> TokenStream {
    if let Some(attr::Protocol::FixedLength(length)) = attr::protocol(&field.attrs) {
        let position = fields.clone().into_iter().position(|f| f == field).unwrap();

        quote! {
            __hints.set_field_length(#position, #length, protocol::hint::LengthPrefixKind::Elements);
        }
    } else {
        quote! { }
    }
}

fn maybe_skip(field: syn::Field) -> Option<TokenStream> {
    if let Some(attr::Protocol::SkipIf(expr)) = attr::protocol(&field.attrs) {
        Some(expr.to_token_stream())
    } else {
        None
    }
}

fn update_hints_after_write<'a>(field: &'a syn::Field,
                                fields: impl IntoIterator<Item=&'a syn::Field> + Clone)
                                -> TokenStream {
    let hint_setters = length_prefix_of(field, fields.clone()).iter().map(|(length_prefix_of, kind, prefix_subfield_names)| {
        let field_name = &field.ident;
        let kind = kind.path_expr();

        quote! {
                __hints.set_field_length(#length_prefix_of,
                                         (self.#field_name #(.#prefix_subfield_names)* ).clone() as usize,
                                         #kind);
        }
    }).collect::<Vec<TokenStream>>();

    if hint_setters.is_empty() {
        quote! {}
    } else {
        quote! {
            #(#hint_setters);*
        }
    }
}

/// If the given field is a length prefix of another field, that other field
/// returned here.
///
/// Returns `None` if the given field is not a disjoint length prefix.
///
/// Returns the field index of the field whose length is specified.
fn length_prefix_of<'a>(field: &'a syn::Field,
                        fields: impl IntoIterator<Item=&'a syn::Field> + Clone)
                        -> Vec<(usize, attr::LengthPrefixKind, Vec<syn::Ident>)> {
    let potential_prefix = field.ident.as_ref();

    let prefixes_of: Vec<&'a Field> = fields.clone().into_iter().filter(|potential_prefix_of| {
        match attr::protocol(&potential_prefix_of.attrs) {
            Some(attr::Protocol::LengthPrefix { ref prefix_field_name, .. }) => {
                if !fields.clone().into_iter().any(|f| f.ident.as_ref() == Some(prefix_field_name)) {
                    panic!("length prefix is invalid: there is no sibling field named '{}", prefix_field_name);
                }

                potential_prefix == Some(prefix_field_name)
            }
            _ => false,
        }
    }).collect();


    prefixes_of.iter()
        .map(|prefix_of| {
            let prefix_of_index = fields.clone().into_iter().position(|f| &f == prefix_of).unwrap();
            match attr::protocol(&prefix_of.attrs).unwrap() {
                attr::Protocol::LengthPrefix { kind, prefix_subfield_names, .. } => {
                    (prefix_of_index, kind.clone(), prefix_subfield_names)
                }
                _ => unreachable!(),
            }
        }).collect()
}

fn write_named_fields(fields_named: &syn::FieldsNamed)
                      -> TokenStream {
    let field_writers: Vec<_> = fields_named.named.iter().map(|field| {
        let field_name = &field.ident;
        // This field may store the length prefix of another field.
        let update_hints = update_hints_after_write(field, &fields_named.named);
        let update_hints_fixed = update_hint_fixed_length(field, &fields_named.named);

        quote! {
            {
                #update_hints_fixed
                let res = protocol::Parcel::write_field(&self. #field_name, __io_writer, __settings, &mut __hints);
                #update_hints
                __hints.next_field();
                res?
            }
        }
    }).collect();

    quote! { #( #field_writers );* }
}

fn read_unnamed_fields(fields_unnamed: &syn::FieldsUnnamed)
                       -> TokenStream {
    let field_initializers: Vec<_> = fields_unnamed.unnamed.iter().map(|field| {
        let field_ty = &field.ty;

        quote! {
            {
                let res: Result<#field_ty, _> = protocol::Parcel::read_field(__io_reader, __settings, &mut __hints);
                __hints.next_field();
                res?
            }
        }
    }).collect();

    quote! { ( #( #field_initializers ),* ) }
}

fn write_unnamed_fields(fields_unnamed: &syn::FieldsUnnamed)
                        -> TokenStream {
    let field_indices = (0..fields_unnamed.unnamed.len()).into_iter().map(syn::Index::from);

    let field_writers: Vec<_> = field_indices.map(|field_index| {
        quote! {
            {
                let res = protocol::Parcel::write_field(&self. #field_index, __io_writer, __settings, &mut __hints);
                __hints.next_field();
                res?
            }
        }
    }).collect();

    quote! { #( #field_writers );* }
}

