use syn::{parse, ItemFn};
use quote::quote;

extern crate proc_macro;
use self::proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn my_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let i: ItemFn = parse(item).unwrap();

    let ident = i.ident;
    let fn_body = i.block;
    let ident_str = syn::LitStr::new(&ident.to_string(), ident.span());

    let test_struct = quote! {
        #[test_case]
        const #ident: SimpleFnTest = SimpleFnTest {
            name: concat!(module_path!(), "::", #ident_str),
            fnc: || {
                #fn_body
            }
        };
    };

    TokenStream::from(test_struct)
}