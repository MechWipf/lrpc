use proc_macro::TokenStream;
mod attribute;
mod derive;

/// Structure and enumeration implement Store trait
#[proc_macro_derive(CommonStore)]
pub fn common_store(input: TokenStream) -> TokenStream {
    derive::common_store(input)
}

/// The format function becomes fn (& mut ByteQue)-> ByteQue
#[proc_macro_attribute]
pub fn fmt_function(_: TokenStream, input: TokenStream) -> TokenStream {
    attribute::fmt_function(input)
}
