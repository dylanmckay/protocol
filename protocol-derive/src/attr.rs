use format;

use syn;

/// Gets the discriminant format of an enum.
pub fn discriminant_format<F: format::Format>(attrs: &[syn::Attribute]) -> Option<F> {
    helper::protocol_meta_name_value_literal("discriminant", attrs).map(helper::expect_lit_str).map(|format_name| {
        match F::from_str(&format_name) {
            Ok(f) => f,
            Err(..) => panic!("invalid enum discriminant format: '{}'", format_name),
        }
    })
}

/// Gets the value of the `repr(type)` attribute.
pub fn repr(attrs: &[syn::Attribute]) -> Option<syn::Ident> {
    attribute::with_ident("repr", attrs)
}

pub fn protocol_variant_discriminator(attrs: &[syn::Attribute]) -> Option<syn::Lit> {
    helper::protocol_meta_nested_named_literal("discriminator", attrs)
}

mod attribute {
    pub fn with_list(name: &str, attrs: &[syn::Attribute]) -> Option<Vec<syn::NestedMeta>> {
        attrs.iter().filter_map(|attr| match attr.interpret_meta() {
            Some(syn::Meta::List(list)) => {
                if list.ident == name { Some(list.nested.into_iter().collect()) } else { None }
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
            syn::NestedMeta::Meta(syn::Meta::Word(ident)) => ident,
            _ => panic!("expected an ident"),
        })
    }
}

mod helper {
    use syn;
    use proc_macro2;

    fn protocol_meta_list(attrs: &[syn::Attribute]) -> Option<syn::MetaList> {
        attrs.iter().filter_map(|attr| match attr.interpret_meta() {
            Some(syn::Meta::List(meta_list)) => {
                if meta_list.ident == syn::Ident::new("protocol", proc_macro2::Span::call_site()) {
                    Some(meta_list)
                } else {
                    // Unrelated attribute.
                    None
                }
            },
            _ => None,
        }).next()
    }

    pub fn protocol_meta_nested_named_literal(name: &str, attrs: &[syn::Attribute]) -> Option<syn::Lit> {
        protocol_meta_list(attrs).and_then(|meta_list| {
            meta_list.nested.into_iter().
                filter_map(|n| match n {
                    syn::NestedMeta::Meta(syn::Meta::List(nested_list)) => {
                        if nested_list.ident == name {
                            if nested_list.nested.len() != 1 {
                                panic!("#[protocol({}(<value>))] attributes can only have one value", name)
                            }

                            match nested_list.nested.into_iter().next().unwrap() {
                                syn::NestedMeta::Literal(lit) => Some(lit),
                                _ => panic!("#[protocol({}(<value>))] values must be valid literals", name),
                            }
                        } else {
                            // irrelevant meta
                            None
                        }
                    },
                    _ => None,
                }).next()
        })
    }

    pub fn protocol_meta_nested_name_values(attrs: &[syn::Attribute]) -> Vec<syn::MetaNameValue> {
        protocol_meta_list(attrs).map(|meta_list| {
            let name_values: Vec<_> = meta_list.nested.iter().
                filter_map(|n| match n {
                    syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) => Some(nv.clone()),
                    _ => None,
                }).collect();
            name_values
        }).unwrap_or_else(|| Vec::new())
    }

    pub fn protocol_meta_name_value_literal(meta_name: &str, attrs: &[syn::Attribute]) -> Option<syn::Lit> {
        protocol_meta_nested_name_values(attrs).iter().filter_map(|name_value| {
            if name_value.ident == syn::Ident::new(meta_name, proc_macro2::Span::call_site()) {
                Some(name_value.lit.clone())
            } else {
                None // Different meta_name
            }
        }).next()
    }

    pub fn expect_lit_str(lit: syn::Lit) -> String {
        match lit {
            syn::Lit::Str(s) => s.value(),
            _ => panic!("expected a string literal"),
        }
    }
}

