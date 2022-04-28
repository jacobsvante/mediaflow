use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MediaflowFile)]
pub fn derive_file_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    // TODO: Check fields agains FileFull
    TokenStream::from(quote!(
        impl crate::MediaflowFile for #name {}
    ))
}

#[proc_macro_derive(MediaflowFolder)]
pub fn derive_folder_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    TokenStream::from(quote!(
        impl crate::MediaflowFolder for #name {}
    ))
}
