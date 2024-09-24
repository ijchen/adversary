mod test_attribute;
mod test_func;

use proc_macro2::TokenStream;
use syn::Ident;
use test_attribute::TestAttribute;
use test_func::TestFunc;

pub fn adv_test(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    // Parse the function declaration
    let func = TestFunc::parse(item)?;

    // Parse the attribute itself, and arguments (like custom generators)
    // let test_identifier_args: Vec<Ident> = func
    //     .non_receiver_args()
    //     .flat_map(|arg| match &*arg.pat {
    //         syn::Pat::Ident(arg) => Some(arg.ident.clone()),
    //         _ => None,
    //     })
    //     .collect();
    // let _attributes = TestAttribute::parse(attr, &test_identifier_args)?;

    Ok(func.into_converted_tokens())
}
