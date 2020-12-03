use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Ident, ItemFn};

#[proc_macro_attribute]
pub fn soln(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn: ItemFn = syn::parse(item).expect("failed to parse input");
    let syn::ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = item_fn;
    let ident_str = sig.ident.to_string();
    let mut sig2 = sig.clone();
    let mut ident_str2 = ident_str.clone();
    ident_str2.push_str("_");
    let ident2 = Ident::new(&ident_str2, Span::call_site());
    sig2.ident = ident2.clone();
    let item_fn2 = syn::ItemFn {
        attrs,
        vis: syn::Visibility::Inherited,
        sig: sig2,
        block: block.clone(),
    };
    let output = quote! {
        #vis #sig {
            println!("=== Running ({}) ===", #ident_str);
            let result = #ident2();
            println!("=== Finished ({}) ===", #ident_str);
            result
        }

        #item_fn2
    };
    output.into()
}
