use proc_macro::TokenStream;
use syn::{DeriveInput, Error};

use crate::consts::{NO_FIELDS, ONLY_NAMED, ONLY_STRUCTS, PARSE_ERROR, TOO_MANY_FIELDS};

fn new_spanned_error(input: &DeriveInput, message: &str) -> TokenStream {
    Error::new_spanned(input, message)
        .to_compile_error()
        .into()
}

pub fn only_named_structs(input: &DeriveInput) -> TokenStream {
    new_spanned_error(input, ONLY_NAMED)
}

pub fn only_structs(input: &DeriveInput) -> TokenStream {
    new_spanned_error(input, ONLY_STRUCTS)
}

pub fn no_fields(input: &DeriveInput) -> TokenStream {
    new_spanned_error(input, NO_FIELDS)
}

pub fn too_many_fields(input: &DeriveInput, item_count: usize) -> TokenStream {
    let message = format!("{TOO_MANY_FIELDS} (found {})", item_count);
    new_spanned_error(input, &message)
}

pub fn parse_error(input: &DeriveInput) -> TokenStream {
    new_spanned_error(input, PARSE_ERROR)
}
