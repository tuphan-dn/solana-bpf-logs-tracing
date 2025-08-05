extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn trace(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = input.sig.ident.to_string();
    let block = input.block;
    let sig = input.sig;
    let vis = input.vis;

    let output = quote! {
        #vis #sig {
            let __result = (|| #block)();
            #[cfg(feature = "tracing")]
            if let Err(ref __e) = __result {
                msg!("`{}` error thrown at {}:{}", #fn_name, file!(), line!());
            }
            __result
        }
    };
    output.into()
}
