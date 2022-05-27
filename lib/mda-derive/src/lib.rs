extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    AttributeArgs,
    FnArg,
    Lit,
    Meta,
    MetaNameValue,
    NestedMeta,
    ReturnType,
    TraitItem,
};

#[proc_macro_attribute]
pub fn rpc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(attr as AttrinbuteArgs);
    match attr_args.as_slice() {
        [] => {},
        [NestedMeta::Meta(meta)] if meta.path().is_ident("name") => return item,
        _ => panic!("Unexpected attr args!")

    }
}
