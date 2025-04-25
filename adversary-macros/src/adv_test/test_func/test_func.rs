use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{
    punctuated::{Pair, Punctuated},
    spanned::Spanned,
    Attribute, Block, Expr, FnArg, Ident, ItemFn, PatType, Token, Visibility,
};

use crate::adv_test::test_attribute::TestAttribute;

use super::Expectation;

#[derive(Debug)]
/// A `syn::ItemFn`, with certain properties verified:
/// - The function is not const, async, unsafe, or extern
/// - The function has at least one argument
/// - The function has no more than twelve arguments
/// - The function does not include any receiver (`self`) arguments
/// - The function is not generic and has no where clause
/// - The function does not include a variadic argument
/// - The return type is one of the following:
///   - <no return type> (implicitly `()`)
///   - [`bool`]
///   - [`Result<(), E>`] for any `E`
pub struct TestFunc {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub fn_token: Token![fn],
    pub ident: Ident,
    pub paren_token: syn::token::Paren,
    pub inputs: Punctuated<PatType, Token![,]>,
    pub output: Expectation,
    pub block: Box<Block>,
}

impl TestFunc {
    pub fn input_identifier_args(&self) -> Vec<Ident> {
        fn extract_identifiers(arg_pat: &syn::Pat) -> Vec<Ident> {
            use syn::Pat as P;
            match arg_pat {
                P::Ident(pat_ident) => vec![pat_ident.ident.clone()],

                P::Paren(pat_paren) => extract_identifiers(&pat_paren.pat),

                P::Reference(pat_reference) => extract_identifiers(&pat_reference.pat),

                P::Tuple(pat_tuple) => pat_tuple
                    .elems
                    .iter()
                    .flat_map(extract_identifiers)
                    .collect(),

                P::Type(pat_type) => extract_identifiers(&pat_type.pat),

                _ => vec![],
            }
        }
        self.inputs
            .iter()
            .flat_map(|arg| extract_identifiers(&arg.pat))
            .collect()
    }

    pub fn parse(item: TokenStream) -> syn::Result<Self> {
        let item_fn: ItemFn = syn::parse2(item)?;

        // Ensure the function isn't `const`
        if let Some(constness) = item_fn.sig.constness {
            return Err(syn::Error::new(
                constness.span(),
                "adversary test functions cannot be `const`",
            ));
        }

        // Ensure the function isn't `async`
        if let Some(asyncness) = item_fn.sig.asyncness {
            return Err(syn::Error::new(
                asyncness.span(),
                "adversary test functions cannot be `async`",
            ));
        }

        // Ensure the function isn't `unsafe`
        if let Some(unsafety) = item_fn.sig.unsafety {
            return Err(syn::Error::new(
                unsafety.span(),
                "adversary test functions cannot be `unsafe`",
            ));
        }

        // Ensure the function isn't `extern`
        if let Some(abi) = item_fn.sig.abi {
            return Err(syn::Error::new(
                abi.span(),
                "adversary test functions cannot be `extern \"...\"",
            ));
        }

        // Ensure the function isn't generic
        // TODO(ichen): could it be worth someday allowing generic tests with
        // a user-provided list of concrete types to apply?
        if !item_fn.sig.generics.params.is_empty() {
            let mut generics = item_fn.sig.generics;
            generics.where_clause = None;
            return Err(syn::Error::new_spanned(
                generics,
                "adversary test functions cannot be generic",
            ));
        }

        // Ensure the function doesn't have a where clause
        if let Some(where_clause) = item_fn.sig.generics.where_clause {
            return Err(syn::Error::new(
                where_clause.span(),
                "adversary test functions cannot have a `where` clause",
            ));
        }

        // Ensure the function has at least one argument
        if item_fn.sig.inputs.is_empty() {
            // TODO(ichen): I'd really like to add some kind of note or info
            // informing the developer that they could just use a normal rust
            // #[test] - needs `#![feature(proc_macro_diagnostics)]`
            // TODO(ichen): look into https://docs.rs/proc-macro2-diagnostics/latest/proc_macro2_diagnostics/
            return Err(syn::Error::new(
                item_fn.sig.paren_token.span.span(),
                "adversary test functions must have at least one argument",
            ));
        }

        // Ensure the function has no more than twelve arguments
        if item_fn.sig.inputs.len() > 12 {
            return Err(syn::Error::new(
                // TODO(ichen): this should really be item_fn.sig.inputs.span(),
                // but that currently only includes the first token in the
                // signature's inputs. Probably an issue with syn/proc_macro's
                // Span, I think this is getting fixed at some point. For now,
                // I'm just using the parenthesis around the inputs, which works
                // as expected.
                item_fn.sig.paren_token.span.span(),
                "adversary test functions must have no more than twelve arguments",
            ));
        }

        // Ensure the function doesn't have any receiver (`self`) arguments
        let inputs = item_fn
            .sig
            .inputs
            .into_pairs()
            .map(|pair| match pair {
                Pair::Punctuated(FnArg::Receiver(receiver), _)
                | Pair::End(FnArg::Receiver(receiver)) => Err(syn::Error::new(
                    receiver.span(),
                    "adversary test functions cannot include receiver (`self`) arguments",
                )),
                Pair::Punctuated(FnArg::Typed(typed_arg), comma) => {
                    Ok(Pair::Punctuated(typed_arg, comma))
                }
                Pair::End(FnArg::Typed(typed_arg)) => Ok(Pair::End(typed_arg)),
            })
            .collect::<Result<_, _>>()?;

        // Ensure the function isn't variadic
        if let Some(variadic) = item_fn.sig.variadic {
            return Err(syn::Error::new(
                variadic.span(),
                "adversary test functions cannot be variadic",
            ));
        }

        let (output, attrs) = Expectation::parse(item_fn.sig.output, item_fn.attrs)?;

        Ok(Self {
            attrs,
            vis: item_fn.vis,
            fn_token: item_fn.sig.fn_token,
            ident: item_fn.sig.ident,
            paren_token: item_fn.sig.paren_token,
            inputs,
            output,
            block: item_fn.block,
        })
    }

    pub fn into_converted_tokens(self, test_attribute: &TestAttribute) -> TokenStream {
        // TODO(ichen): despaghettify this code, it is not readable at all rn

        let Self {
            attrs,
            vis,
            fn_token,
            ident,
            paren_token,
            inputs,
            output,
            block,
        } = self;

        let paren_token = quote_spanned! { paren_token.span.span() => () };

        let inner_ret = match &output {
            Expectation::DoesNotPanic
            | Expectation::Panics
            | Expectation::PanicsWithMessage { .. } => quote! {},
            Expectation::ReturnsTrue => quote! { -> ::std::primitive::bool },
            Expectation::ReturnsOk { err_ty } => {
                quote! { -> ::std::result::Result<(), #err_ty> }
            }
        };

        // TODO: not do this weird ownership cheat
        // From future me: what weird ownership cheat?? The `.collect()`?
        let arg_types = inputs.iter().map(|arg| &arg.ty).collect::<Vec<_>>();

        let arg_idents: Vec<Ident> = (0..inputs.len())
            .map(|n| format_ident!("arg_{n}"))
            .collect();

        let test_name = ident.to_string();

        fn get_custom_generator(
            arg_pat: &syn::Pat,
            arg_ty: &syn::Type,
            generators: &HashMap<Ident, Expr>,
        ) -> Option<TokenStream> {
            use syn::Pat as P;
            match arg_pat {
                P::Ident(pat_ident) => generators
                    .get(&pat_ident.ident)
                    .map(|generator| quote! { #generator }),
                P::Paren(pat_paren) => get_custom_generator(&pat_paren.pat, arg_ty, generators),
                P::Reference(pat_reference) => {
                    get_custom_generator(&pat_reference.pat, arg_ty, generators)
                }
                P::Tuple(pat_tuple) => {
                    let individual_gens = pat_tuple
                        .elems
                        .iter()
                        .zip(match arg_ty {
                            syn::Type::Tuple(type_tuple) => &type_tuple.elems,
                            _ => todo!(),
                        })
                        .map(|(elem, elem_arg_ty)| {
                            get_custom_generator(elem, elem_arg_ty, generators)
                                .unwrap_or_else(|| quote! { ::adversary::any::<#elem_arg_ty>() })
                        })
                        .collect::<Vec<TokenStream>>();

                    Some(quote! { (#(#individual_gens),*) })
                }
                P::Type(pat_type) => get_custom_generator(&pat_type.pat, arg_ty, generators),
                _ => None,
            }
        }
        let generators = inputs
            .iter()
            .map(|arg| {
                let arg_ty = &arg.ty;
                get_custom_generator(&arg.pat, arg_ty, &test_attribute.generators)
                    .unwrap_or_else(|| quote! { ::adversary::any::<#arg_ty>() })
            })
            .collect::<Vec<_>>();
        let generator = quote! { (#(#generators),*) };

        let test_run = match &output {
            Expectation::DoesNotPanic => quote! {
                ::adversary::run_test_panics(
                    |(#(#arg_idents),*)| inner_test(#(#arg_idents),*),
                    generator,
                    &mut rng,
                )
            },
            Expectation::Panics => {
                return quote! { compile_error!("adversary tests that should panic are not yet implemented"); }
            }
            Expectation::PanicsWithMessage {
                expected_substring: _,
            } => {
                return quote! { compile_error!("adversary tests that should panic with a message are not yet implemented"); }
            }
            Expectation::ReturnsTrue => quote! {
                ::adversary::run_test(
                    |(#(#arg_idents),*)| inner_test(#(#arg_idents),*),
                    generator,
                    &mut rng,
                )
            },
            Expectation::ReturnsOk { err_ty: _ } => {
                // NOTE(ichen): should use the specialization hack to turn the
                // error type into a string - first Display, then Debug, then a
                // default message for types which don't impl either.
                // https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html
                return quote! { compile_error!("adversary tests that return a Result<(), _> are not yet implemented") };
            }
        };

        quote! {
            #[test]
            #(#attrs)*
            #vis #fn_token #ident #paren_token -> ::std::process::ExitCode {
                fn inner_test(#inputs) #inner_ret #block

                let mut generator = #generator;
                let mut rng = ::adversary::rand::thread_rng();

                let run_result = #test_run;

                let ::std::result::Result::Err(mut report) = run_result else {
                    return ::std::process::ExitCode::SUCCESS;
                };
                report.test_name = ::std::option::Option::Some(::std::string::String::from(#test_name));

                // NOTE(ichen): Uses a cute specialization hack to convert the
                // generic `T` value into a `String` - through `Display` if
                // possible, then `Debug` if possible, and finally falling back
                // to a default message for types which don't implement either.
                //
                // See:
                // https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html
                //
                // TODO(ichen): figure out why specifying this type is necessary
                //                                 vvvvvvvvvvvvvvvvvvvv
                let specialized_to_string = |(#(#arg_idents),*): &(#(#arg_types),*)| {
                    struct Wrap<'a, T>(&'a T);

                    trait ViaDisplay { fn stringify(&self) -> ::std::string::String; }
                    impl<'a, T: ::std::fmt::Display> ViaDisplay for &&Wrap<'a, T> {
                        fn stringify(&self) -> ::std::string::String { ::std::format!("{}", self.0) }
                    }

                    trait ViaDebug { fn stringify(&self) -> ::std::string::String; }
                    impl<'a, T: ::std::fmt::Debug> ViaDebug for &Wrap<'a, T> {
                        fn stringify(&self) -> ::std::string::String { ::std::format!("{:?}", self.0) }
                    }

                    trait Fallback { fn stringify(&self) -> ::std::string::String; }
                    impl<'a, T> Fallback for Wrap<'a, T> {
                        fn stringify(&self) -> ::std::string::String { ::std::format!("<{}>", ::std::any::type_name::<T>()) }
                    }

                    ::std::format!("{}", <[_]>::join(&[#(
                        (&&&Wrap(#arg_idents)).stringify()
                    ),*], ", "))
                };
                ::std::eprintln!("{}", report.render::<::adversary::report::renderer::Plaintext>(specialized_to_string));

                ::std::process::ExitCode::FAILURE
            }
        }
    }
}
