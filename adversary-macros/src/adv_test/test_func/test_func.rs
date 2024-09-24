use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    punctuated::{Pair, Punctuated},
    spanned::Spanned,
    Attribute, Block, FnArg, Ident, ItemFn, PatType, ReturnType, Signature, Token, Type,
    Visibility,
};

use super::TestFuncOutput;

#[derive(Debug)]
/// A `syn::ItemFn`, with certain properties verified:
/// - The function is not const, async, unsafe, or extern
/// - The function does not include any receiver (`self`) arguments
/// - The function is not generic and has no where clause
/// - The function does not include a variadic argument
/// - The return type is one of the following:
///   - <no return type> (implicitly `()`)
///   - [`bool`]
///   - [`Result<(), E>`] for any `E`
pub struct TestFunc {
    // pub item_fn: ItemFn,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub fn_token: Token![fn],
    pub ident: Ident,
    pub paren_token: syn::token::Paren,
    pub inputs: Punctuated<PatType, Token![,]>,
    pub output: TestFuncOutput,
    pub block: Box<Block>,
}

impl TestFunc {
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

        if let Some(variadic) = item_fn.sig.variadic {
            return Err(syn::Error::new(
                variadic.span(),
                "adversary test functions cannot include a variadic argument",
            ));
        }

        let (output, attrs) = TestFuncOutput::parse(item_fn.sig.output, item_fn.attrs)?;

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

    // pub fn non_receiver_args(&self) -> impl Iterator<Item = &PatType> {
    //     self.item_fn.sig.inputs.iter().map(|arg| match arg {
    //         syn::FnArg::Receiver(_) => {
    //             unreachable!("receiver args checked for in constructor for Self")
    //         }
    //         syn::FnArg::Typed(arg) => arg,
    //     })
    // }

    pub fn into_converted_tokens(self) -> TokenStream {
        // let ItemFn {
        //     attrs,
        //     vis,
        //     sig:
        //         Signature {
        //             constness,
        //             asyncness,
        //             unsafety,
        //             abi,
        //             fn_token,
        //             ident,
        //             generics,
        //             paren_token,
        //             inputs,
        //             variadic,
        //             output,
        //         },
        //     block,
        // } = self.item_fn;

        // quote! {
        //     #[test]
        //     #attrs
        //     #vis
        //     fn
        // }
        todo!()
    }
}
