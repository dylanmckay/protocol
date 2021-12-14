use crate::{codegen, plan};
use proc_macro2::{Span, TokenStream};
use syn;

/// Generates code that reads one of a set of
/// parcel variants and returns an expression
/// of the same type as the enum.
pub fn write_variant(plan: &plan::Enum)
    -> TokenStream {
    let enum_name = &plan.ident;
    let discriminator_ty = plan.discriminant();

    let variant_match_branches: Vec<_> = plan.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let discriminator_ref_expr = variant.discriminator_ref_expr();

        let write_discriminator_stmt = quote! { <#discriminator_ty as protocol::Parcel>::write(#discriminator_ref_expr, __io_writer, __settings)?; };

        let (binding_names, fields_pattern) = bind_fields_pattern(variant_name, &variant.fields);

        quote!(#enum_name :: #fields_pattern => {
            #write_discriminator_stmt

            #(
                protocol::Parcel::write_field(#binding_names, __io_writer, __settings, &mut __hints)?;
            )*
        })
    }).collect();

    quote! {
        match *self {
            #(#variant_match_branches,)*
            _ => panic!("unknown discriminator"), // FIXME: this should not be a panic
        }
    }
}

// TODO: write a read_variant function.

pub fn read_variant(plan: &plan::Enum)
    -> TokenStream {
    let enum_name = &plan.ident;
    let discriminator_ty = plan.discriminant();
    let discriminator_var = syn::Ident::new("discriminator", Span::call_site());
    let discriminator_for_pattern_matching = plan.matchable_discriminator_expr(discriminator_var.clone());

    let discriminator_match_branches = plan.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let discriminator_literal = variant.discriminator_literal();
        let initializer = codegen::read_enum_fields(&variant.fields);

        quote! {
            #discriminator_literal => {
                #enum_name::#variant_name # initializer
            }
        }

    });

    quote! {
        {
            let discriminator: #discriminator_ty = protocol::Parcel::read_field(__io_reader, __settings, &mut __hints)?;

            match #discriminator_for_pattern_matching {
                #(#discriminator_match_branches,)*
                unknown_discriminator => {
                    return Err(protocol::ErrorKind::UnknownEnumDiscriminator(
                        stringify!(#enum_name), format!("{:?}", unknown_discriminator),
                    ).into());
                },
            }
        }
    }
}

/// Generates code for a pattern that binds a set of fields by reference.
///
/// Returns a tuple of the pattern tokens and the field binding names.
pub fn bind_fields_pattern(parent_name: &syn::Ident,
                           fields: &syn::Fields)
    -> (Vec<syn::Ident>, TokenStream) {
    match *fields {
        syn::Fields::Named(ref fields_named) => {
            let field_names: Vec<_> = fields_named.named.iter().map(|f| f.ident.clone().unwrap()).collect();
            let field_name_refs = fields_named.named.iter().map(|f| &f.ident).map(|n| quote! { ref #n });

            (field_names, quote! {
                #parent_name { #( #field_name_refs ),* }
            })
        },
        syn::Fields::Unnamed(ref fields_unnamed) => {
            let binding_names: Vec<_> = (0..fields_unnamed.unnamed.len()).into_iter()
                .map(|i| syn::Ident::new(&format!("field_{}", i), proc_macro2::Span::call_site()))
                .collect();

            let field_refs: Vec<_> = binding_names.iter().map(|i| quote! { ref #i }).collect();

            (binding_names, quote! {
                #parent_name ( #( #field_refs ),* )
            })
        },
        syn::Fields::Unit => (Vec::new(), quote!(#parent_name)),
    }
}

