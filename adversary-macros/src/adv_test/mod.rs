mod test_attribute;
mod test_func;

use proc_macro2::TokenStream;
use test_attribute::TestAttribute;
use test_func::TestFunc;

pub fn adv_test(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    // Parse the function declaration
    let func = TestFunc::parse(item)?;

    // Parse the attribute itself, and arguments (like custom generators)
    let attributes = TestAttribute::parse(attr, &func.input_identifier_args())?;

    Ok(func.into_converted_tokens(&attributes))
}
