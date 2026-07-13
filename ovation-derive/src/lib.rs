use proc_macro::TokenStream;
// use proc_macro2::TokenStream as TokenStream2;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(CommandSet, attributes(contexts, delegates))]
pub fn command_set(input: TokenStream) -> TokenStream {
    let _input: DeriveInput = parse_macro_input!(input);
    // TODO: impl(input) -> Result<TokenStream2, syn::Error>
    TokenStream::default()
}

#[proc_macro_derive(CommandContext, attributes(set))]
pub fn command_context(input: TokenStream) -> TokenStream {
    let _input: DeriveInput = parse_macro_input!(input);
    // TODO: impl(input) -> Result<TokenStream2, syn::Error>
    TokenStream::default()
}

