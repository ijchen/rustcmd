mod execute;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn execute(attr: TokenStream, item: TokenStream) -> TokenStream {
    execute::execute(attr.into(), item.into())
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}
