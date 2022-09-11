#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use] extern crate quote;

mod attr;
mod codegen;
mod format;
mod plan;

use proc_macro::TokenStream;

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
        syn::Data::Enum(ref e) => {
            let plan = plan::Enum::new(ast, e);

            let mut stream = impl_parcel_for_enum(&plan, ast);
            stream.extend(impl_enum_for_enum(&plan, ast));
            stream
        },
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
    let read_fields = codegen::read_fields(&strukt.fields);
    let write_fields = codegen::write_fields(&strukt.fields);

    impl_trait_for(ast, quote!(protocol::Parcel), quote! {
        const TYPE_NAME: &'static str = stringify!(#strukt_name);

        #[allow(unused_variables)]
        fn read_field(__io_reader: &mut io::Read,
                      __settings: &protocol::Settings,
                      _: &mut protocol::hint::Hints)
            -> protocol::Result<Self> {
            // Each type gets its own hints.
            let mut __hints = protocol::hint::Hints::default();
            __hints.begin_fields();

            Ok(#strukt_name # read_fields)
        }

        #[allow(unused_variables)]
        fn write_field(&self, __io_writer: &mut io::Write,
                       __settings: &protocol::Settings,
                       _: &mut protocol::hint::Hints)
            -> protocol::Result<()> {
            // Each type gets its own hints.
            let mut __hints = protocol::hint::Hints::default();
            __hints.begin_fields();

            #write_fields
            Ok(())
        }
    })
}

/// Generates a `Parcel` trait implementation for an enum.
fn impl_parcel_for_enum(plan: &plan::Enum,
                        ast: &syn::DeriveInput)
    -> proc_macro2::TokenStream {

    let enum_name = &plan.ident;
    let read_variant = codegen::enums::read_variant(plan);
    let write_variant = codegen::enums::write_variant(plan);

    impl_trait_for(ast, quote!(protocol::Parcel), quote! {
        const TYPE_NAME: &'static str = stringify!(#enum_name);

        #[allow(unused_variables)]
        fn read_field(__io_reader: &mut io::Read,
                      __settings: &protocol::Settings,
                      _: &mut protocol::hint::Hints)
            -> protocol::Result<Self> {
            // Each type gets its own hints.
            let mut __hints = protocol::hint::Hints::default();
            __hints.begin_fields();

            Ok(#read_variant)
        }

        #[allow(unused_variables)]
        fn write_field(&self, __io_writer: &mut io::Write,
                       __settings: &protocol::Settings,
                       _: &mut protocol::hint::Hints)
            -> protocol::Result<()> {
            // Each type gets its own hints.
            let mut __hints = protocol::hint::Hints::default();
            __hints.begin_fields();

            #write_variant

            Ok(())
        }
    })
}

fn impl_enum_for_enum(plan: &plan::Enum,
                      ast: &syn::DeriveInput)
    -> proc_macro2::TokenStream {
    let enum_ident = &plan.ident;
    let discriminant = plan.discriminant();

    let variant_matchers = plan.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let discriminator = variant.discriminator_expr();
        let fields_expr = variant.ignore_fields_pattern_expr();

        quote!(#enum_ident::#variant_ident #fields_expr => {
            #discriminator
        })
    });

    impl_trait_for(ast, quote!(protocol::Enum), quote!(
        type Discriminant = #discriminant;

        fn discriminator(&self) -> Self::Discriminant {
            match *self {
                #(#variant_matchers)*
            }
        }
    ))
}

/// Wraps a stream of tokens in an anonymous constant block.
///
/// Inside this block, the protocol crate accessible.
fn anonymous_constant_block(description: &str,
                            item_name: &syn::Ident,
                            body: proc_macro2::TokenStream)
    -> proc_macro2::TokenStream {
    let anon_const_name = syn::Ident::new(&format!("__{}_FOR_{}",
                                                   description.replace(" ", "_").replace("::", "_"),
                                                   item_name.to_owned()),
                                          proc_macro2::Span::call_site());

    quote! {
        #[allow(non_upper_case_globals)]
        const #anon_const_name: () = {
            extern crate protocol;
            use std::io;

            #body
        };
    }
}

fn impl_trait_for(ast: &syn::DeriveInput,
                  trait_name: proc_macro2::TokenStream,
                  impl_body: proc_macro2::TokenStream)
    -> proc_macro2::TokenStream {
    let item_name = &ast.ident;
    let description = format!("impl {}", trait_name);

    let (generics, where_predicates) = build_generics(ast);
    let (generics, where_predicates) = (&generics, where_predicates);

    anonymous_constant_block(&description, item_name, quote! {
        impl < #(#generics),* > #trait_name for #item_name < #(#generics),* >
            where #(#where_predicates),* {
            #impl_body
        }
    })
}

