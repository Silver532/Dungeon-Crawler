use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn timeit(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemFn = parse_macro_input!(item as ItemFn);
    let name: String = input.sig.ident.to_string();
    let vis: &syn::Visibility = &input.vis;
    let sig: &syn::Signature = &input.sig;
    let block: &Box<syn::Block> = &input.block;

    let expanded: proc_macro2::TokenStream = quote! {
        #vis #sig {
            #[cfg(feature = "timing")]
            let __timeit_start__ = std::time::Instant::now();

            let __timeit_result__ = (|| #block)();

            #[cfg(feature = "timing")]
            crate::timing::record(#name, __timeit_start__.elapsed());

            __timeit_result__
        }
    };

    TokenStream::from(expanded)
}