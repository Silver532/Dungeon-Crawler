use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, LitStr, parse_macro_input};

#[proc_macro_attribute]
pub fn timeit(attr: TokenStream, item: TokenStream) -> TokenStream {
    let stage: String = parse_macro_input!(attr as LitStr).value();
    let input: ItemFn = parse_macro_input!(item as ItemFn);
    let name: String = input.sig.ident.to_string();
    let vis: &syn::Visibility = &input.vis;
    let sig: &syn::Signature = &input.sig;
    let block: &syn::Block = &input.block;

    let expanded: proc_macro2::TokenStream = quote! {
        #vis #sig {
            let __timeit_start__ = std::time::Instant::now();

            let __timeit_result__ = (|| #block)();

            crate::timing::record(#stage, #name, __timeit_start__.elapsed());

            __timeit_result__
        }
    };
    TokenStream::from(expanded)
}