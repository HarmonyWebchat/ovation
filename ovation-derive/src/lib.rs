use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(CommandSet, attributes(contexts, delegates))]
pub fn command_set(input: TokenStream) -> TokenStream {
    // stub
    fn command_set_impl(
        DeriveInput { attrs: _, vis: _, ident: _, generics: _, data: _ }: DeriveInput
    ) -> Result<TokenStream2, TokenStream2> {
        Ok(TokenStream2::default())
    }

    match command_set_impl(parse_macro_input!(input)) {
        Ok(tokens) => tokens.into(),
        Err(tokens) => tokens.into(),
    }
}

#[proc_macro_derive(CommandContext, attributes(set))]
pub fn command_context(input: TokenStream) -> TokenStream {
    // stub
    fn command_context_impl(
        DeriveInput { attrs: _, vis: _, ident: _, generics: _, data: _ }: DeriveInput
    ) -> Result<TokenStream2, TokenStream2> {
        Ok(TokenStream2::default())
    }

    match command_context_impl(parse_macro_input!(input)) {
        Ok(tokens) => tokens.into(),
        Err(tokens) => tokens.into(),
    }
}

