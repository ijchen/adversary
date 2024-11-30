use std::collections::HashMap;

use proc_macro2::TokenStream;
use syn::{parse::Parser, punctuated::Punctuated, spanned::Spanned, Expr, Ident, Meta, Token};

#[derive(Debug)]
pub struct TestAttribute {
    #[expect(dead_code, reason = "will be used, code not implemented")]
    pub generators: HashMap<Ident, Expr>,
}

impl TestAttribute {
    pub fn parse(attributes: TokenStream, test_identifier_args: &[Ident]) -> syn::Result<Self> {
        let attributes =
            Parser::parse2(Punctuated::<Meta, Token![,]>::parse_terminated, attributes)?;

        let mut generators = HashMap::with_capacity(attributes.len());

        for meta in attributes {
            match meta {
                Meta::Path(_) | Meta::List(_) => {
                    return Err(syn::Error::new(meta.span(), "invalid meta item argument"))
                }

                // NameValue (<path> = <expr>) are always generators
                Meta::NameValue(name_value) => {
                    let ident = name_value.path.require_ident()?.clone();
                    let expr = name_value.value;

                    // Generate an error if this identifier doesn't appear as an
                    // argument in the test function
                    if !test_identifier_args.contains(&ident) {
                        return Err(syn::Error::new(
                            ident.span(),
                            format!("no argument named `{ident}` in the test function"),
                        ));
                    }

                    // Generate and error if we've already seen this argument
                    if generators.contains_key(&ident) {
                        return Err(syn::Error::new(ident.span(), "duplicate argument"));
                    }

                    generators.insert(ident, expr);
                }
            }
        }

        Ok(Self { generators })
    }
}
