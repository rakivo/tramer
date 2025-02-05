extern crate proc_macro;

use quote::quote;
use proc_macro::TokenStream;
use syn::{ItemFn, LitStr, parse_macro_input};

#[proc_macro_attribute]
pub fn tramer(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ItemFn { attrs, vis, sig, block, .. } = parse_macro_input!(item as ItemFn);
    let ref fn_name = sig.ident;

    let (time_unit_tokens, time_unit) = if !attr.is_empty() {
        let time_unit_tokens = parse_macro_input!(attr as LitStr);
        let time_unit = time_unit_tokens.value().to_string();
        let time_unit_tokens = match time_unit.as_str() {
            "millis"  => quote! { __start__.elapsed().as_millis() },
            "secs" => quote! { __start__.elapsed().as_secs_f64() },
            "nanos"   => quote! { __start__.elapsed().as_nanos() },
            _ => panic!("unsupported time unit: use 'millis', 'seconds', or 'nanos'"),
        };
        (time_unit_tokens, time_unit)
    } else {
        (quote! { __start__.elapsed().as_millis() }, "millis".to_owned())
    };

    TokenStream::from(quote! {
        #(#attrs)*
        #vis #sig {
            let __start__ = std::time::Instant::now();
            let ret = (|| #block)();
            println!{
                "{name} took {elapsed:.3} {time_unit}",
                name = stringify!(#fn_name),
                elapsed = #time_unit_tokens,
                time_unit = #time_unit
            };
            ret
        }
    })
}
