use {attr, format};
use syn;
use proc_macro2::{Span, TokenStream};

/// The default type used to represent
/// integer discriminators unless explicitly
/// overriden.
const DEFAULT_INT_DISCRIMINANT: &'static str = "u32";


/// The first integer discriminator assigned,
/// unless an explicit discriminator is given
/// in the first variant.
const DEFAULT_FIRST_INT_DISCRIMINATOR: usize = 1;

/// A plan for a Parcel implementation for an enum.
pub struct Enum {
    /// The name of the enum type.
    pub ident: syn::Ident,
    /// The enum format.
    pub explicit_format: Option<format::Enum>,
    /// The `#[repr(..)]` attribute.
    pub repr_attr: Option<syn::Ident>,
    pub variants: Vec<EnumVariant>,
}

/// An enum variant.
pub struct EnumVariant {
    /// The name of this variant.
    pub ident: syn::Ident,
    /// The optional `#[protocol(discriminator(<type>))]` attribute of this variant.
    pub explicit_discriminator_attr: Option<syn::Lit>,
    /// The optional `Variant = <int value>` value.
    pub explicit_int_discriminator_equals: Option<syn::Lit>,
    /// The actual discriminator value used by this variant.
    ///
    /// Filled in by the `resolve` function.
    pub actual_discriminator: Option<syn::Lit>,
    /// The fields of the enum.
    pub fields: syn::Fields,
}

impl Enum {
    /// Creates a layout plan for an enum.
    pub fn new(ast: &syn::DeriveInput,
               e: &syn::DataEnum) -> Enum {
        let mut plan = Enum {
            ident: ast.ident.clone(),
            repr_attr: attr::repr(&ast.attrs),
            explicit_format: attr::discriminant_format::<format::Enum>(&ast.attrs),
            variants: e.variants.iter().map(|variant| {
                let equals_discriminant = match variant.discriminant.clone().map(|a| a.1) {
                    Some(syn::Expr::Lit(expr_lit)) => Some(expr_lit.lit),
                    Some(_) => panic!("'VariantName = <expr>' can only be used with literals"),
                    None => None,
                };

                EnumVariant {
                    ident: variant.ident.clone(),
                    explicit_discriminator_attr: attr::protocol_variant_discriminator(&variant.attrs),
                    explicit_int_discriminator_equals: equals_discriminant,
                    actual_discriminator: None,
                    fields: variant.fields.clone(),
                }
            }).collect(),
        };
        plan.resolve();
        plan
    }


    pub fn format(&self) -> format::Enum {
        if let Some(ref explicit_format) = self.explicit_format {
            explicit_format.clone()
        } else { // no explicit format given, use default
            format::Enum::default()
        }
    }

    /// Gets the type used for the discriminant.
    pub fn discriminant(&self) -> syn::Ident {
        match self.repr_attr.clone() {
            // An explicit discriminant via `#[repr(ty)]`.
            Some(ty) => ty,
            // Use the default discriminant.
            None => match self.format() {
                format::Enum::StringDiscriminator => {
                    syn::Ident::new("String", Span::call_site())
                },
                format::Enum::IntegerDiscriminator => {
                    syn::Ident::new(DEFAULT_INT_DISCRIMINANT, Span::call_site())
                },
            },
        }
    }

    /// Gets an expression that can be used in as the RHS in pattern matching.
    pub fn matchable_discriminator_expr(&self,
                                        variable_ident: syn::Ident)
        -> TokenStream {
        match self.format() {
            format::Enum::IntegerDiscriminator => quote!(#variable_ident),
            format::Enum::StringDiscriminator => quote!(&#variable_ident[..]),
        }
    }

    pub fn resolve(&mut self) {
        let mut current_default_int_discriminator = DEFAULT_FIRST_INT_DISCRIMINATOR;
        let format = self.format().clone();

        for variant in self.variants.iter_mut() {
            let actual_discriminator: syn::Lit = match variant.explicit_discriminator() {
                Some(explicit_discriminator) => explicit_discriminator.clone(),
                None => match format {
                    format::Enum::StringDiscriminator => {
                        // By default, assign string discriminators as the name of
                        // the variant itself.
                        syn::LitStr::new(&variant.ident.to_string(), Span::call_site()).into()
                    },
                    format::Enum::IntegerDiscriminator => {
                        // By default, assign integer discriminators the value of the
                        // last discriminator plus one.
                        syn::LitInt::new(current_default_int_discriminator as _,
                                         syn::IntSuffix::None,
                                         Span::call_site()).into()
                    },
                },
            };

            // Change the default int discriminator value if relevant.
            if let syn::Lit::Int(ref discriminator_value) = actual_discriminator {
                current_default_int_discriminator = discriminator_value.value() as usize + 1;
            }

            // Store the actual discriminator in memory for later use.
            variant.actual_discriminator = Some(actual_discriminator);
        }
    }
}

impl EnumVariant {
    /// Gets the discriminator of the variant.
    pub fn discriminator_literal(&self) -> &syn::Lit {
        self.actual_discriminator.as_ref().expect("discriminator has not been resolved yet")
    }

    /// Gets the discriminator explicitly specified in the code, if any.
    ///
    /// Handles all possible ways of explicitly setting discriminators.
    pub fn explicit_discriminator(&self) -> Option<&syn::Lit> {
        match (self.explicit_discriminator_attr.as_ref(), self.explicit_int_discriminator_equals.as_ref()) {
            // When both are specified, prefer the #[protocol] attribute
            (Some(attr), Some(_)) => Some(attr),
            // When one is specified, use it.
            (Some(lit), None) | (None, Some(lit)) => Some(lit),
            (None, None) => None,
        }
    }

    /// Gets an expression representing the discriminator.
    pub fn discriminator_expr(&self) -> TokenStream {
        match self.discriminator_literal() {
            s @ syn::Lit::Str(..) => quote!(#s.to_owned()),
            i @ syn::Lit::Int(..) => quote!(#i),
            _ => unreachable!(),
        }
    }

    /// Gets an expression representing a reference to
    /// the discriminator.
    pub fn discriminator_ref_expr(&self) -> TokenStream {
        match self.discriminator_literal() {
            s @ syn::Lit::Str(..) => quote!(&#s.to_owned()),
            i @ syn::Lit::Int(..) => quote!(&#i),
            _ => unreachable!(),
        }
    }

    /// Gets a pattern expression that ignores the fields of
    /// this variant.
    pub fn ignore_fields_pattern_expr(&self) -> TokenStream {
        match self.fields {
            syn::Fields::Named(..) => quote!({ .. }),
            syn::Fields::Unnamed(..) => quote!((..)),
            syn::Fields::Unit => quote!(),
        }
    }
}

