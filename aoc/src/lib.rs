use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_attribute]
pub fn soln(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn: syn::ItemFn = syn::parse(item).expect("failed to parse input");
    let syn::ItemFn { attrs: _attrs, vis, sig, block } = item_fn;
    let ident = sig.ident.to_string();
    let stmts = block.stmts;
    let output = quote!{vterm
        #vis #sig {
            println!("=== Running ({}) ===", #ident);
            #(#stmts)*
            println!("=== Finished ({}) ===", #ident);
        }
    };
    output.into()
}
