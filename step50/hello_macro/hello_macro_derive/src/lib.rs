
extern crate proc_macro;


use proc_macro::TokenStream;

use quote::quote;
use syn;


#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {

    // Construct a representation of rust code as a syntax tree
    // then we can manipulate

    let ast = syn::parse(input).unwrap();

    impl_hello_macro(&ast)
}



fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {

    let name = &ast.ident;

    let gen = quote! {

        impl HelloMacro for #name{
            fn hello_macro() {

                println!("Hello, Macro! name is {}!", stringify!(#name))

            }
        }

    };
    gen.into()

}