use proc_macro::*;
use quote::quote;

#[proc_macro_attribute]
pub fn test(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::ItemFn);
    let name = input.sig.ident;
    let body = input.block;
    let new_input = quote! {
        #[test_case]
        pub fn #name() {
            crate::println!("Running test: {}", stringify!(#name));
            #body
            crate::println!("[ok]");
        }
    };
    new_input.into()
}
