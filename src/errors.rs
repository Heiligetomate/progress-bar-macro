use proc_macro::TokenStream;
use syn::{DeriveInput, Error};

use crate::consts::{NO_FIELDS, ONLY_NAMED, ONLY_STRUCTS, TOO_MANY_FIELDS};

pub fn only_named_structs(input: &DeriveInput) -> TokenStream {
    return Error::new_spanned(&input.ident, ONLY_NAMED)
        .to_compile_error()
        .into();
}

pub fn only_structs(input: &DeriveInput) -> TokenStream {
    return Error::new_spanned(&input.ident, ONLY_STRUCTS)
        .to_compile_error()
        .into();
}

pub fn no_fields(input: &DeriveInput) -> TokenStream {
    return Error::new_spanned(&input.ident, NO_FIELDS)
        .to_compile_error()
        .into();
}

pub fn too_many_fields(input: &DeriveInput, item_count: usize) -> TokenStream {
    return Error::new_spanned(&input.ident, format!("{TOO_MANY_FIELDS} (found {})", item_count))
        .to_compile_error()
        .into();
}
