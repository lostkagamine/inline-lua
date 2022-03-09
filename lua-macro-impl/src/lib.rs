#![feature(proc_macro_span)]
#![feature(proc_macro_hygiene)]

extern crate proc_macro;
use proc_macro::TokenStream;

#[macro_use]
extern crate quote;

#[proc_macro]
pub fn lua(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    let mut span = tokens.next().unwrap().span();

    while let Some(tk) = tokens.next() {
        span = span.join(tk.span()).unwrap();
    }

    let src = span.source_text().unwrap();

    quote!( lua_macro::run_lua::<()>(#src); ).into()
}

#[proc_macro]
pub fn lua_eval(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    let mut span = tokens.next().unwrap().span();

    while let Some(tk) = tokens.next() {
        span = span.join(tk.span()).unwrap();
    }

    let src = span.source_text().unwrap();

    quote!( lua_macro::run_lua::<_>(#src) ).into()
}