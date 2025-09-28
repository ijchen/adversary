use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::Parser, punctuated::Punctuated, spanned::Spanned, Expr, Ident, Meta, MetaNameValue,
    Token,
};

#[derive(Debug)]
pub struct TestAttribute {
    pub generators: HashMap<Ident, Expr>,
    pub config: Option<Expr>,
}

impl TestAttribute {
    pub fn parse(attributes: TokenStream, test_identifier_args: &[Ident]) -> syn::Result<Self> {
        let attributes =
            Parser::parse2(Punctuated::<Meta, Token![,]>::parse_terminated, attributes)?;

        let mut generators = HashMap::with_capacity(attributes.len());
        let mut config = None;

        for meta in attributes {
            match meta {
                Meta::Path(_) => {
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

                Meta::List(list) => match list.path.get_ident() {
                    Some(ident) if ident == "config" => {
                        // TODO(ichen): this points at the wrong thing, and it's almost 4:00 AM so
                        // this is a future me problem
                        let list_span = list.span();

                        if config.is_some() {
                            return Err(syn::Error::new(list_span, "duplicate config"));
                        }

                        // config(<expr of type TestConfig>)
                        if let Ok(expr) = syn::parse2(list.tokens.clone()) {
                            config = Some(expr);
                            continue;
                        }

                        // config(field1 = value, field2 = value, ...)
                        if let Ok(fields) = Parser::parse2(
                            Punctuated::<MetaNameValue, Token![,]>::parse_terminated,
                            list.tokens,
                        ) {
                            let fields = fields
                                .into_iter()
                                .map(|field| {
                                    let name = field.path.require_ident()?.clone();
                                    let value = field.value;
                                    Ok(quote! { #name: #value })
                                })
                                .collect::<syn::Result<Vec<_>>>()?;

                            config = Some(
                                syn::parse2(quote! {
                                    ::adversary::test_runners::TestConfig {
                                        #(#fields,)*
                                        ..::adversary::test_runners::TestConfig::default()
                                    }
                                })
                                .expect("hard-coded code didn't parse - this is a bug"),
                            );
                            continue;
                        };

                        return Err(syn::Error::new(list_span, "invalid config"));
                    }
                    Some(_) => {
                        return Err(syn::Error::new(list.span(), "invalid meta list argument"));
                    }
                    None => {
                        return Err(syn::Error::new(list.span(), "invalid meta item argument"));
                    }
                },
            }
        }

        Ok(Self { generators, config })
    }
}
