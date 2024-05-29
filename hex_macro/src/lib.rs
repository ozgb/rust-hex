use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn hex(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr).value();

    // Validate and decode the hex string
    match hex_core::decode(input) {
        Ok(decoded) => {
            let len = decoded.len();
            // Convert decoded bytes to a byte array
            let byte_array = quote! {
                {
                    const DATA: [u8; #len] = [#(#decoded),*];
                    DATA
                }
            };
            byte_array.into()
        }
        Err(_) => panic!("Invalid hex string"),
    }
}
