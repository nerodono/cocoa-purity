use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[cfg(feature = "map_enum")]
#[proc_macro_error]
#[proc_macro_attribute]
pub fn map_enum(args: TokenStream, body: TokenStream) -> TokenStream {
    macros::map_enum::map_enum(args, body)
}

mod macros;
