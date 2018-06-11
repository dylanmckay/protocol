#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

mod format;

use proc_macro::TokenStream;

#[proc_macro_derive(Protocol, attributes(protocol_discriminant))]
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

fn impl_parcel_for_struct(ast: &syn::DeriveInput,
                          strukt: &syn::DataStruct) -> proc_macro2::TokenStream {
    let strukt_name = &ast.ident;
    let anon_const_name = syn::Ident::new(&format!("__IMPL_PARCEL_FOR_{}", strukt_name.to_owned()), proc_macro2::Span::call_site());

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

                    impl protocol::Parcel for #strukt_name {
                        #[allow(unused_variables)]
                        fn read(read: &mut io::Read)
                            -> Result<Self, protocol::Error> {
                            Ok(#strukt_name {
                                #(
                                    #field_names: protocol::Parcel::read(read)?
                                ),*
                            })
                        }

                        #[allow(unused_variables)]
                        fn write(&self, write: &mut io::Write)
                            -> Result<(), protocol::Error> {
                            #( protocol::Parcel::write(&self. #field_names, write )?; )*
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
                quote!{ protocol::Parcel::read(read)? }
            });

            quote! {
                #[allow(non_upper_case_globals)]
                const #anon_const_name: () = {
                    extern crate protocol;
                    use std::io;

                    impl protocol::Parcel for #strukt_name {
                        #[allow(unused_variables)]
                        fn read(read: &mut io::Read)
                            -> Result<Self, protocol::Error> {
                            Ok(#strukt_name(
                                #(#field_expressions),*
                            ))
                        }

                        #[allow(unused_variables)]
                        fn write(&self, write: &mut io::Write)
                            -> Result<(), protocol::Error> {
                            #( protocol::Parcel::write(&self. #field_numbers, write )?; )*
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
                        fn read(_: &mut io::Read) -> Result<Self, protocol::Error> {
                            Ok(#strukt_name)
                        }

                        fn write(&self, _: &mut io::Write)
                            -> Result<(), protocol::Error> {
                            Ok(())
                        }
                    }
                };
            }
        },
    }
}

fn protocol_discriminant<F: format::Format>(ast: &syn::DeriveInput) -> Option<F> {
    ast.attrs.iter().filter_map(|attr| match attr.interpret_meta() {
        Some(syn::Meta::NameValue(syn::MetaNameValue { ident, lit, .. })) => {
            if ident == syn::Ident::new("protocol_discriminant", proc_macro2::Span::call_site()) {
                match lit {
                    syn::Lit::Str(s) => {
                        match F::from_str(&s.value()) {
                            Ok(f) => Some(f),
                            Err(..) => None,
                        }
                    },
                    _ => panic!("unexpected type"),
                }
            } else {
                None
            }
        },
        _ => None,
    }).next()
}

fn impl_parcel_for_enum(ast: &syn::DeriveInput,
                        e: &syn::DataEnum) -> proc_macro2::TokenStream {
    let enum_name = &ast.ident;
    let anon_const_name = syn::Ident::new(&format!("__IMPL_PARCEL_FOR_{}", ast.ident), proc_macro2::Span::call_site());

    let format = protocol_discriminant::<format::Enum>(ast).unwrap_or(format::Enum::IntegerDiscriminator);

    let variant_writers = e.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        let discriminator = format.discriminator(e, variant);
        let write_discriminator = quote! { protocol::Parcel::write(&#discriminator, __io_writer)?; };

        match variant.fields {
            syn::Fields::Named(ref fields_named) => {
                let field_names = fields_named.named.iter().map(|f| &f.ident);
                let field_name_refs = fields_named.named.iter().map(|f| &f.ident).map(|n| quote! { ref #n });

                quote! {
                    #enum_name :: #variant_name { #( #field_name_refs ),* } => {
                        #write_discriminator

                        #( protocol::Parcel::write(#field_names, __io_writer)?; )*
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
                        #( protocol::Parcel::write(#binding_names, __io_writer)?; )*
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

    let variant_readers = e.variants.iter().map(|ref variant| {
        let variant_name = &variant.ident;
        let discriminator = format.discriminator_variant_for_pattern_matching(e, variant);

        match variant.fields {
            syn::Fields::Named(ref fields_named) => {
                let field_names = fields_named.named.iter().map(|f| &f.ident);

                quote! {
                    #discriminator => Ok(#enum_name :: #variant_name {
                        #( #field_names : protocol::Parcel::read(__io_reader)? ),*
                    })
                }
            },
            syn::Fields::Unnamed(ref fields_unnamed) => {
                let binding_names: Vec<_> = (0..fields_unnamed.unnamed.len()).into_iter()
                    .map(|i| syn::Ident::new(&format!("field_{}", i), proc_macro2::Span::call_site()))
                    .collect();

                let field_readers = binding_names.iter().map(|_| {
                    quote! {
                        protocol::Parcel::read(__io_reader)?
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

    let discriminator_type = format.discriminator_type();
    let discriminator_for_pattern_matching = format.discriminator_for_pattern_matching();
    quote! {
        #[allow(non_upper_case_globals)]
        const #anon_const_name: () = {
            extern crate protocol;
            use std::io;

            impl protocol::Parcel for #enum_name {
                #[allow(unused_variables)]
                fn read(__io_reader: &mut io::Read) -> Result<Self, protocol::Error> {
                    let discriminator: #discriminator_type = protocol::Parcel::read(__io_reader)?;
                    match #discriminator_for_pattern_matching {
                        #(#variant_readers,)*
                        _ => panic!("unknown discriminator"),
                    }
                }

                #[allow(unused_variables)]
                fn write(&self, __io_writer: &mut io::Write)
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

