pub mod enums;

use proc_macro2::TokenStream;
use syn;

pub fn read_fields(fields: &syn::Fields)
    -> TokenStream {
    match *fields {
        syn::Fields::Named(ref fields_named) => {
            let field_names: Vec<_> = fields_named.named.iter().map(|field| {
                field.ident.clone().unwrap()
            }).collect();

            read_named_fields(&field_names[..])
        },
        syn::Fields::Unnamed(ref fields) => read_unnamed_fields(fields.unnamed.len()),
        syn::Fields::Unit => quote!(),
    }
}

pub fn write_fields(fields: &syn::Fields)
    -> TokenStream {
    match *fields {
        syn::Fields::Named(ref fields_named) => {
            let field_names: Vec<_> = fields_named.named.iter().map(|field| {
                field.ident.clone().unwrap()
            }).collect();

            write_named_fields(&field_names[..])
        },
        syn::Fields::Unnamed(ref fields) => write_unnamed_fields(fields.unnamed.len()),
        syn::Fields::Unit => quote!(),
    }
}

/// Generates code that builds a initializes
/// an item with named fields by parsing
/// each of the fields.
///
/// Returns  `{ ..field initializers.. }`.
fn read_named_fields(field_names: &[syn::Ident])
    -> TokenStream {
    let field_initializers: Vec<_> = field_names.iter().map(|field_name| {
        quote! {
            #field_name : {
                let res = protocol::Parcel::read(__io_reader, __settings, &mut __hints);
                __hints.next_field();
                res?
            }
        }
    }).collect();

    quote! { { #( #field_initializers ),* } }
}

fn write_named_fields(field_names: &[syn::Ident])
    -> TokenStream {
    let field_writers: Vec<_> = field_names.iter().map(|field_name| {
        quote! {
            {
                protocol::Parcel::write(&self. #field_name, __io_writer, __settings )?;
            }
        }
    }).collect();

    quote! { #( #field_writers );* }
}

fn read_unnamed_fields(field_count: usize)
    -> TokenStream {
    let field_initializers: Vec<_> = (0..field_count).into_iter().map(|_| {
        quote! {
            {
                let res = protocol::Parcel::read(__io_reader, __settings, &mut __hints);
                __hints.next_field();
                res?
            }
        }
    }).collect();

    quote! { ( #( #field_initializers ),* ) }
}

fn write_unnamed_fields(field_count: usize)
    -> TokenStream {
    let field_indices = (0..field_count).into_iter().map(syn::Index::from);

    let field_writers: Vec<_> = field_indices.map(|field_index| {
        quote! {
            {
                protocol::Parcel::write(&self. #field_index, __io_writer, __settings )?;
            }
        }
    }).collect();

    quote! { #( #field_writers );* }
}

