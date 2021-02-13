use crate::format::{self, Format};

use proc_macro2::{Span, TokenStream};
use syn;

#[derive(Debug)]
pub enum Protocol {
    DiscriminantFormat(format::Enum),
    Discriminator(syn::Lit),
    LengthPrefix {
        kind: LengthPrefixKind,
        prefix_field_name: syn::Ident,
        prefix_subfield_names: Vec<syn::Ident>,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LengthPrefixKind {
    Bytes,
    Elements,
}

impl LengthPrefixKind {
    /// Gets a path to the length prefix in the protocol crate.
    pub fn path_expr(&self) -> TokenStream {
        match *self {
            LengthPrefixKind::Bytes => quote!(protocol::hint::LengthPrefixKind::Bytes),
            LengthPrefixKind::Elements => quote!(protocol::hint::LengthPrefixKind::Elements),
        }
    }
}

/// Gets the value of the `repr(type)` attribute.
pub fn repr(attrs: &[syn::Attribute]) -> Option<syn::Ident> {
    attribute::with_ident("repr", attrs)
}

pub fn protocol(attrs: &[syn::Attribute])
    -> Option<Protocol> {
    let meta_list = attrs.iter().filter_map(|attr| match attr.parse_meta() {
        Ok(syn::Meta::List(meta_list)) => {
            if meta_list.path.get_ident() == Some(&syn::Ident::new("protocol", proc_macro2::Span::call_site())) {
                Some(meta_list)
            } else {
                // Unrelated attribute.
                None
            }
        },
        _ => None,
    }).next();

    let meta_list: syn::MetaList = if let Some(meta_list) = meta_list { meta_list } else { return None };
    let mut nested_metas = meta_list.nested.into_iter();

    match nested_metas.next() {
        Some(syn::NestedMeta::Meta(syn::Meta::List(nested_list))) => {
            match &nested_list.path.get_ident().expect("meta is not an ident").to_string()[..] {
                // #[protocol(length_prefix(<kind>(<prefix field name>)))]
                "length_prefix" => {
                    let nested_list = expect::meta_list::nested_list(nested_list)
                                            .expect("expected a nested list");
                    let prefix_kind = match &nested_list.path.get_ident().expect("nested list is not an ident").to_string()[..] {
                        "bytes" => LengthPrefixKind::Bytes,
                        "elements" => LengthPrefixKind::Elements,
                        invalid_prefix => panic!("invalid length prefix type: '{}'", invalid_prefix),
                    };

                    let length_prefix_expr = expect::meta_list::single_element(nested_list).unwrap();
                    let (prefix_field_name, prefix_subfield_names) = match length_prefix_expr {
                        syn::NestedMeta::Lit(syn::Lit::Str(s)) => {
                            let mut parts: Vec<_> = s.value()
                                                     .split(".")
                                                     .map(|s| syn::Ident::new(s, Span::call_site()))
                                                     .collect();

                            if parts.len() < 1 {
                                panic!("there must be at least one field mentioned");
                            }

                            let field_ident = parts.remove(0);
                            let subfield_idents = parts.into_iter().collect();

                            (field_ident, subfield_idents)
                        },
                        syn::NestedMeta::Meta(syn::Meta::Path(path)) => match path.get_ident() {
                            Some(field_ident) => (field_ident.clone(), Vec::new()),
                            None => panic!("path is not an ident"),
                        },
                        _ => panic!("unexpected format for length prefix attribute"),
                    };

                    Some(Protocol::LengthPrefix { kind: prefix_kind, prefix_field_name, prefix_subfield_names })
                },
                "discriminator" => {
                    let literal = expect::meta_list::single_literal(nested_list)
                                        .expect("expected a single literal");
                    Some(Protocol::Discriminator(literal))
                },
                name => panic!("#[protocol({})] is not valid", name),
            }
        },
        Some(syn::NestedMeta::Meta(syn::Meta::NameValue(name_value))) => {
            match name_value.path.get_ident() {
                Some(ident) => {
                    match &ident.to_string()[..] {
                        // #[protocol(discriminant = "<format_name>")]
                        "discriminant" => {
                            let format_kind = match name_value.lit {
                                syn::Lit::Str(s) => match format::Enum::from_str(&s.value()) {
                                    Ok(f) => f,
                                    Err(()) => panic!("invalid enum discriminant format: '{}", s.value()),
                                },
                                _ => panic!("discriminant format mut be string"),
                            };

                            Some(Protocol::DiscriminantFormat(format_kind))
                        },
                        ident => panic!("expected 'discriminant' but got '{}", ident),
                    }
                },
                None => panic!("expected 'discriminant' but the parsed string was not even an identifier"),
            }
        },
        _ => panic!("#[protocol(..)] attributes cannot be empty"),
    }
}

mod expect {
    pub mod meta_list {
        pub fn nested_list(list: syn::MetaList)
            -> Result<syn::MetaList, ()> {
            assert!(list.nested.len() == 1, "list should only have one item");
            match list.nested.into_iter().next().unwrap() {
                syn::NestedMeta::Meta(syn::Meta::List(nested)) => Ok(nested),
                _ => Err(()),
            }
        }

        /// Expects a list with a single element.
        pub fn single_element(list: syn::MetaList)
            -> Result<syn::NestedMeta, ()> {
            assert!(list.nested.len() == 1, "list should only have one item");
            Ok(list.nested.into_iter().next().unwrap())
        }

        /// A single word `name(literal)`.
        pub fn single_literal(list: syn::MetaList)
            -> Result<syn::Lit, ()> {
            single_element(list).and_then(|nested| match nested {
                syn::NestedMeta::Lit(lit) => Ok(lit),
                _ => Err(()),
            })
        }
    }
}

mod attribute {
    pub fn with_list(name: &str, attrs: &[syn::Attribute]) -> Option<Vec<syn::NestedMeta>> {
        attrs.iter().filter_map(|attr| match attr.parse_meta() {
            Ok(syn::Meta::List(list)) => {
                match list.path.get_ident() {
                    Some(ident) if ident == name => Some(list.nested.into_iter().collect()),
                    _ => None,
                }
            },
            _ => None,
        }).next()
    }

    pub fn with_unitary_list(name: &str, attrs: &[syn::Attribute]) -> Option<syn::NestedMeta> {
        with_list(name, attrs).map(|list| {
            if list.len() != 1{ panic!("expected only one meta inside list but found {}", list.len()); }
            list.into_iter().next().unwrap()
        })
    }

    pub fn with_ident(name: &str, attrs: &[syn::Attribute]) -> Option<syn::Ident> {
        with_unitary_list(name, attrs).map(|nested| match nested {
            syn::NestedMeta::Meta(syn::Meta::Path(path)) => match path.get_ident() {
                Some(ident) => ident.clone(),
                None => panic!("expected an ident"),
            },
            _ => panic!("expected an ident"),
        })
    }
}

