#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

mod attr;
mod format;
mod plan;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;

#[proc_macro_derive(Protocol, attributes(protocol))]
pub fn protocol(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    // Build the impl
    let gen = impl_parcel(&ast);

    // Return the generated impl
    gen.to_string().parse().expect("Could not parse generated parcel impl")
}

// The `Parcel` trait is used for data that can be sent/received.
fn impl_parcel(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    match ast.data {
        syn::Data::Struct(ref s) => impl_parcel_for_struct(ast, s),
        syn::Data::Enum(ref e) => impl_parcel_for_enum(ast, e),
        syn::Data::Union(..) => unimplemented!(),
    }
}

/// Builds generics for a new impl.
///
/// Returns `(generics, where_predicates)`
fn build_generics(ast: &syn::DeriveInput) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) {
    use quote::ToTokens;

    let mut where_predicates = Vec::new();
    let mut generics = Vec::new();

    generics.extend(ast.generics.type_params().map(|t| {
        let (ident, bounds) = (&t.ident, &t.bounds);
        where_predicates.push(quote!(#ident : protocol::Parcel + #bounds));
        quote!(#ident)
    }));

    generics.extend(ast.generics.lifetimes().enumerate().map(|(i, _)| {
        let letter = ('a' as u8 + i as u8) as char;
        quote!(#letter)
    }));

    if let Some(where_clause) = ast.generics.where_clause.clone() {
        where_predicates.push(where_clause.predicates.into_token_stream());
    }

    assert!(ast.generics.const_params().next().is_none(),
            "constant parameters are not supported yet");

    (generics, where_predicates)
}

fn impl_parcel_for_struct(ast: &syn::DeriveInput,
                          strukt: &syn::DataStruct) -> proc_macro2::TokenStream {
    let strukt_name = &ast.ident;
    let anon_const_name = syn::Ident::new(&format!("__IMPL_PARCEL_FOR_{}", strukt_name.to_owned()), proc_macro2::Span::call_site());

    let (generics, where_predicates) = build_generics(ast);
    let (generics, where_predicates) = (&generics, where_predicates);

    match strukt.fields {
        syn::Fields::Named(ref fields_named) => {
            let field_names: Vec<_> = fields_named.named.iter().map(|field| {
                &field.ident
            }).collect();
            let field_names = &field_names[..];

            quote! {
                #[allow(non_upper_case_globals)]
                const #anon_const_name: () = {
                    extern crate protocol;
                    use std::io;

                    impl < #(#generics),* > protocol::Parcel for #strukt_name < #(#generics),* >
                        where #(#where_predicates),* {
                        const TYPE_NAME: &'static str = stringify!(#strukt_name);

                        #[allow(unused_variables)]
                        fn read(read: &mut io::Read,
                                __settings: &protocol::Settings)
                            -> Result<Self, protocol::Error> {
                            Ok(#strukt_name {
                                #(
                                    #field_names: protocol::Parcel::read(read, __settings)?
                                ),*
                            })
                        }

                        #[allow(unused_variables)]
                        fn write(&self, write: &mut io::Write,
                                 __settings: &protocol::Settings)
                            -> Result<(), protocol::Error> {
                            #( protocol::Parcel::write(&self. #field_names, write, __settings )?; )*
                            Ok(())
                        }
                    }
                };
            }
        },
        syn::Fields::Unnamed(ref fields_unnamed) => {
            let field_numbers: Vec<_> = (0..fields_unnamed.unnamed.len()).into_iter().map(syn::Index::from).collect();
            let field_numbers = &field_numbers[..];

            let field_expressions = field_numbers.iter().map(|_| {
                quote!{ protocol::Parcel::read(read, __settings)? }
            });

            quote! {
                #[allow(non_upper_case_globals)]
                const #anon_const_name: () = {
                    extern crate protocol;
                    use std::io;

                    impl < #(#generics),* > protocol::Parcel for #strukt_name < #(#generics),* >
                        where #(#where_predicates),* {
                        const TYPE_NAME: &'static str = stringify!(#strukt_name);

                        #[allow(unused_variables)]
                        fn read(read: &mut io::Read,
                                __settings: &protocol::Settings)
                            -> Result<Self, protocol::Error> {
                            Ok(#strukt_name(
                                #(#field_expressions),*
                            ))
                        }

                        #[allow(unused_variables)]
                        fn write(&self, write: &mut io::Write,
                                 __settings: &protocol::Settings)
                            -> Result<(), protocol::Error> {
                            #( protocol::Parcel::write(&self. #field_numbers, write, __settings )?; )*
                            Ok(())
                        }
                    }
                };
            }
        },
        syn::Fields::Unit => {
            quote! {
                #[allow(non_upper_case_globals)]
                const #anon_const_name: () = {
                    extern crate protocol;
                    use std::io;

                    impl protocol::Parcel for #strukt_name {
                        const TYPE_NAME: &'static str = stringify!(#strukt_name);

                        fn read(_: &mut io::Read,
                                _: &protocol::Settings) -> Result<Self, protocol::Error> {
                            Ok(#strukt_name)
                        }

                        fn write(&self, _: &mut io::Write, _: &protocol::Settings)
                            -> Result<(), protocol::Error> {
                            Ok(())
                        }
                    }
                };
            }
        },
    }
}

/// Generates a `Parcel` trait implementation for an enum.
fn impl_parcel_for_enum(ast: &syn::DeriveInput,
                        e: &syn::DataEnum) -> proc_macro2::TokenStream {
    let plan = plan::Enum::new(ast, e);

    let enum_name = &ast.ident;
    let anon_const_name = syn::Ident::new(&format!("__IMPL_PARCEL_FOR_{}", ast.ident), proc_macro2::Span::call_site());
    let discriminator_ty = plan.discriminant();

    let discriminator_var = syn::Ident::new("discriminator", Span::call_site());

    let variant_writers = plan.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let discriminator = variant.discriminator();
        let discriminator_ref_expr = plan.discriminator_ref_expr(discriminator.into_token_stream());

        let write_discriminator = quote! { <#discriminator_ty as protocol::Parcel>::write(#discriminator_ref_expr, __io_writer, __settings)?; };

        match variant.fields {
            syn::Fields::Named(ref fields_named) => {
                let field_names = fields_named.named.iter().map(|f| &f.ident);
                let field_name_refs = fields_named.named.iter().map(|f| &f.ident).map(|n| quote! { ref #n });

                quote! {
                    #enum_name :: #variant_name { #( #field_name_refs ),* } => {
                        #write_discriminator

                        #( protocol::Parcel::write(#field_names, __io_writer, __settings)?; )*
                    }
                }
            },
            syn::Fields::Unnamed(ref fields_unnamed) => {
                let binding_names: Vec<_> = (0..fields_unnamed.unnamed.len()).into_iter()
                    .map(|i| syn::Ident::new(&format!("field_{}", i), proc_macro2::Span::call_site()))
                    .collect();

                let field_refs: Vec<_> = binding_names.iter().map(|i| quote! { ref #i }).collect();

                quote! {
                    #enum_name :: #variant_name ( #( #field_refs ),* ) => {
                        #write_discriminator
                        #( protocol::Parcel::write(#binding_names, __io_writer, __settings)?; )*
                    }
                }
            },
            syn::Fields::Unit => {
                quote!{
                    #enum_name :: #variant_name => {
                        #write_discriminator;
                    }
                }
            },
        }
    });

    let variant_readers = plan.variants.iter().map(|ref variant| {
        let variant_name = &variant.ident;
        let discriminator = variant.discriminator();

        match variant.fields {
            syn::Fields::Named(ref fields_named) => {
                let field_names = fields_named.named.iter().map(|f| &f.ident);

                quote! {
                    #discriminator => Ok(#enum_name :: #variant_name {
                        #( #field_names : protocol::Parcel::read(__io_reader, __settings)? ),*
                    })
                }
            },
            syn::Fields::Unnamed(ref fields_unnamed) => {
                let binding_names: Vec<_> = (0..fields_unnamed.unnamed.len()).into_iter()
                    .map(|i| syn::Ident::new(&format!("field_{}", i), proc_macro2::Span::call_site()))
                    .collect();

                let field_readers = binding_names.iter().map(|_| {
                    quote! {
                        protocol::Parcel::read(__io_reader, __settings)?
                    }
                });

                quote! {
                    #discriminator => Ok(#enum_name :: #variant_name (
                        #(#field_readers),*
                    ))
                }
            },
            syn::Fields::Unit => {
                quote! {
                    #discriminator => Ok(#enum_name :: #variant_name)
                }
            },
        }
    });

    let (generics, where_predicates) = build_generics(ast);
    let (generics, where_predicates) = (&generics, where_predicates);

    let discriminator_for_pattern_matching = plan.matchable_discriminator_expr(discriminator_var.clone());
    quote! {
        #[allow(non_upper_case_globals)]
        const #anon_const_name: () = {
            extern crate protocol;
            use std::io;

            impl < #(#generics),* > protocol::Parcel for #enum_name < #(#generics),* >
                where #(#where_predicates),* {
                const TYPE_NAME: &'static str = stringify!(#enum_name);

                #[allow(unused_variables)]
                fn read(__io_reader: &mut io::Read,
                        __settings: &protocol::Settings) -> Result<Self, protocol::Error> {
                    let discriminator: #discriminator_ty = protocol::Parcel::read(__io_reader, __settings)?;
                    match #discriminator_for_pattern_matching {
                        #(#variant_readers,)*
                        _ => panic!("unknown discriminator"),
                    }
                }

                #[allow(unused_variables)]
                fn write(&self, __io_writer: &mut io::Write,
                         __settings: &protocol::Settings)
                    -> Result<(), protocol::Error> {
                    match *self {
                        #(#variant_writers),*
                    }

                    Ok(())
                }
            }
        };
    }
}

