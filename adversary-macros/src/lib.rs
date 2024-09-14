mod adv_test;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn adv_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    adv_test::adv_test(attr.into(), item.into()).into()
}
