use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, ItemFn, PatType};

#[derive(Debug)]
/// A `syn::ItemFn`, with certain properties verified:
/// - The function signature does not include any receiver (`self`) arguments
pub struct TestFunc {
    pub item_fn: ItemFn,
}

impl TestFunc {
    pub fn parse(item: TokenStream) -> syn::Result<Self> {
        let item_fn: ItemFn = syn::parse2(item)?;

        // Ensure the function doesn't have any receiver (`self`) arguments
        for argument in item_fn.sig.inputs.iter() {
            if matches!(argument, syn::FnArg::Receiver(_)) {
                return Err(syn::Error::new(
                    argument.span(),
                    "test functions cannot include receiver (`self`) arguments",
                ));
            }
        }

        Ok(Self { item_fn })
    }

    pub fn non_receiver_args(&self) -> impl Iterator<Item = &PatType> {
        self.item_fn.sig.inputs.iter().map(|arg| match arg {
            syn::FnArg::Receiver(_) => {
                unreachable!("receiver args checked for in constructor for Self")
            }
            syn::FnArg::Typed(arg) => arg,
        })
    }

    pub fn into_converted_tokens(self) -> TokenStream {
        let mut func = self.item_fn;

        func.sig.inputs.clear();

        quote! {
            #[test]
            #func
        }
    }
}
