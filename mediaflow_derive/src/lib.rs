use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MediaflowFile)]
pub fn derive_file_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    // TODO: Check fields against FileFull
    TokenStream::from(quote!(
        impl ::mediaflow::MediaflowFile for #name {}
    ))
}

#[proc_macro_derive(MediaflowFolder)]
pub fn derive_folder_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    // TODO: Check fields against FolderFull
    TokenStream::from(quote!(
        impl ::mediaflow::MediaflowFolder for #name {}
    ))
}

#[proc_macro_derive(MediaflowFileDownload)]
pub fn derive_download_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    // TODO: Check fields against FileDownloadFull
    TokenStream::from(quote!(
        impl ::mediaflow::MediaflowFileDownload for #name {}
    ))
}

#[proc_macro_derive(MediaflowFormat)]
pub fn derive_format_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    // TODO: Check fields against FormatFull
    TokenStream::from(quote!(
        impl ::mediaflow::MediaflowFormat for #name {}
    ))
}
