#![recursion_limit = "128"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Protocol)]
pub fn protocol(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    // Build the impl
    let gen = impl_parcel(&ast);

    // Return the generated impl
    gen.to_string().parse().expect("Could not parse generated parcel impl")
}

// The `Parcel` trait is used for data that can be sent/received.
fn impl_parcel(ast: &syn::DeriveInput) -> quote::Tokens {
    match ast.data {
        syn::Data::Struct(ref s) => impl_parcel_for_struct(ast, s),
        syn::Data::Enum(ref e) => impl_parcel_for_enum(ast, e),
        syn::Data::Union(..) => unimplemented!(),
    }
}

fn impl_parcel_for_struct(ast: &syn::DeriveInput,
                          strukt: &syn::DataStruct) -> quote::Tokens {
    let strukt_name = &ast.ident;
    let anon_const_name = syn::Ident::from(format!("__IMPL_PARCEL_FOR_{}", strukt_name.to_owned()));

    match strukt.fields {
        syn::Fields::Named(ref fields_named) => {
            let field_names: Vec<_> = fields_named.named.iter().map(|field| {
                field.ident
            }).collect();
            let field_names = &field_names[..];

            quote! {
                #[allow(non_upper_case_globals)]
                const #anon_const_name: () = {
                    extern crate protocol;
                    use std::io;

                    impl protocol::Parcel for #strukt_name {
                        fn read(read: &mut io::Read)
                            -> Result<Self, protocol::Error> {
                            Ok(#strukt_name {
                                #(
                                    #field_names: protocol::Parcel::read(read)?
                                ),*
                            })
                        }

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
                        fn read(read: &mut io::Read)
                            -> Result<Self, protocol::Error> {
                            Ok(#strukt_name(
                                #(#field_expressions),*
                            ))
                        }

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
                        fn read(read: &mut io::Read) -> Result<Self, protocol::Error> {
                            Ok(#strukt_name)
                        }

                        fn write(&self, write: &mut io::Write)
                            -> Result<(), protocol::Error> {
                            Ok(())
                        }
                    }
                };
            }
        },
    }
}

fn impl_parcel_for_enum(_ast: &syn::DeriveInput,
                        _e: &syn::DataEnum) -> quote::Tokens {
    unimplemented!();
}

