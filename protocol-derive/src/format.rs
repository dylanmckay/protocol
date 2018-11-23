//! Different protocol formats.

use attr;
use syn;
use proc_macro2::Span;

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
                let discriminator = attr::protocol_variant_discriminator(&variant.attrs)
                                       // Otherwise, figure out the discriminator.
                                       .unwrap_or_else(|| {
                    match variant.discriminant {
                        Some((_, syn::Expr::Lit(syn::ExprLit { ref lit, .. }))) => lit.clone(),
                        Some(_) => panic!("unknown discriminator"),
                        // If no explicit discriminant exists, use the default
                        None => {
                            let variant_index = e.variants.iter().position(|v| v.ident == variant.ident)
                                .expect("variant not a part of enum");
                            let default_discriminator = variant_index as u64 + 1;

                            syn::LitInt::new(default_discriminator, syn::IntSuffix::None,
                                             Span::call_site()).into()
                        },
                    }
                });

                quote!( #discriminator )
            },
            Enum::StringDiscriminator => {
                let variant_name = attr::protocol_variant_discriminator(&variant.attrs)
                                 .unwrap_or_else(|| syn::LitStr::new(&variant.ident.to_string(), Span::call_site()).into());
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
                let variant_name = attr::protocol_variant_discriminator(&variant.attrs)
                    .unwrap_or_else(|| syn::LitStr::new(&variant.ident.to_string(), Span::call_site()).into());
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

