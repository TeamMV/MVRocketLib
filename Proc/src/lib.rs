use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};
#[proc_macro_attribute]
pub fn main(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input function
    let input_fn = parse_macro_input!(input as ItemFn);

    // Extract the function name
    let fn_name = &input_fn.sig.ident;

    let output = quote! {
        #input_fn

        #[no_mangle]
        pub extern "C" fn mvrocket_launch(api: *const mvrocketlib::API, id: *const mvrocketlib::Uuid) {
            mvrocketlib::init(api, id);
            let rpcout = mvrocketlib::RpcOut::new();
            let orig_stdout = std::io::stdout();
            let orig_stdout_handle = orig_stdout.lock();

            std::io::set_print(Some(Box::new(rpcout)));
            #fn_name();
        }
    };

    TokenStream::from(output)
}