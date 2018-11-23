//! Different protocol formats.

use attr;
use syn;

pub const DEFAULT_INT_DISCRIMINATOR_TYPE: &'static str = "u32";

pub const DEFAULT_ENUM_DISCRIMINATOR_FORMAT: Enum = Enum::StringDiscriminator;

/// Represents a format.
pub trait Format : Clone {
    /// From a string.
    fn from_str(s: &str) -> Result<Self, ()>;
}

/// The enum protocol format.
#[derive(Clone, Debug, PartialEq)]
pub enum Enum {
    /// The enum is transmitted by using the 1-based index of the enum variant.
    IntegerDiscriminator,
    /// The enum is transmitted by using the name of the variant.
    StringDiscriminator,
}

impl Enum {
    /// Gets the discriminator of an enum variant.
    pub fn discriminator(&self, e: &syn::DataEnum,
                         variant: &syn::Variant) -> ::proc_macro2::TokenStream {
        let allow_explicit_discriminators = match *self {
            Enum::IntegerDiscriminator => true,
            _ => false,
        };

        if !allow_explicit_discriminators {
            for variant in e.variants.iter() {
                if let Some(_) = variant.discriminant {
                    panic!("only enums with integer discriminants may use explicit discriminants: '{}",
                           quote!(#variant).to_string());
                }
            }
        }

        match *self {
            Enum::IntegerDiscriminator => {
                let variant_index = e.variants.iter().position(|v| v.ident == variant.ident).expect("variant not a part of enum");
                let prior_variants: Vec<_> = e.variants.iter().collect();
                let previous_discriminator = prior_variants.into_iter().rev().filter_map(|variant| {
                    match variant.discriminant {
                        Some((_, syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(ref n), .. }))) => Some(n.value()),
                        _ => None,
                    }
                }).next()
                  .unwrap_or_else(|| variant_index as u64 + 1); // incase no explicit discriminators
                let default_discriminator = previous_discriminator + 1;

                let discriminator = match variant.discriminant {
                    Some((_, syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(ref n), .. }))) => n.clone(),
                    Some(_) => panic!("unknown discriminator"),
                    // If no explicit discriminant exists
                    None => syn::LitInt::new(variant_index as u64 + 1, syn::IntSuffix::None,
                                             proc_macro2::Span::call_site()),
                };

                quote!( #discriminator )
            },
            Enum::StringDiscriminator => {
                let variant_name = attr::name(&variant.attrs).unwrap_or_else(|| variant.ident.to_string());
                quote! { String::from(#variant_name) }
            },
        }
    }

    pub fn discriminator_for_pattern_matching(&self) -> ::proc_macro2::TokenStream {
        match *self {
            Enum::IntegerDiscriminator => quote!(discriminator),
            Enum::StringDiscriminator => quote!(&discriminator[..]),
        }
    }

    pub fn discriminator_variant_for_pattern_matching(&self, e: &syn::DataEnum,
                                                      variant: &syn::Variant) -> ::proc_macro2::TokenStream {
        match *self {
            Enum::IntegerDiscriminator => self.discriminator(e, variant),
            Enum::StringDiscriminator => {
                let variant_name = attr::name(&variant.attrs).unwrap_or_else(|| variant.ident.to_string());
                quote! { #variant_name }
            },
        }
    }
}

impl Format for Enum {
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "integer" => Ok(Enum::IntegerDiscriminator),
            "string" => Ok(Enum::StringDiscriminator),
            _ => Err(()),
        }
    }
}

