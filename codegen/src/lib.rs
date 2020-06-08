#![recursion_limit = "128"]
extern crate proc_macro;
use {
    self::proc_macro::TokenStream,
    proc_macro2,
    quote::*,
    syn::parse::{Parse, ParseStream, Result},
    syn::punctuated::Punctuated,
    syn::{parenthesized, braced, Field, Expr, Type, parse_macro_input, token, Ident, Token},
    heck,
};


mod conf;
mod genstruct;
mod util;

use toml;

use std::io::prelude::*;

use crate::conf::Conf;


use heck::CamelCase;
use std::fs::File;
use std::io::Read;

const CONF_FILE_PATH : &'static str = "confs/method.toml";


#[proc_macro]
pub fn gen_struct(input: TokenStream) -> TokenStream {

    genstruct::struct_to_tokenstream(input)
}

#[proc_macro]
pub fn gen_struct_by_conf(_input: TokenStream) -> TokenStream {
    genstruct::struct_by_conf_to_tokenstream(_input)
}