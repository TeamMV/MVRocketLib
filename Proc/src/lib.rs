use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};
#[proc_macro_attribute]
pub fn main(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    let fn_name = &input_fn.sig.ident;

    let output = quote! {
        #input_fn

        #[no_mangle]
        pub extern "C" fn mvrocket_launch() {
            #fn_name();
        }
    };

    TokenStream::from(output)
}