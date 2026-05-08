use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Meta, parse_macro_input, punctuated::Punctuated, token::Comma};

mod error_messages;
mod errors;

const LABEL_NAME: &str = "max_value";

fn get_progress_bar_fields(fields: &Punctuated<Field, Comma>) -> Vec<&Field> {
    fields
        .iter()
        .filter(|f| if let syn::Type::Path(type_path) = &f.ty { type_path.path.is_ident("ProgressBar") } else { false })
        .collect()
}

fn check_progress_bar_field_count(input: &DeriveInput, len: usize) -> Option<TokenStream> {
    match len {
        0 => Some(errors::no_progress_bars_found(&input)),
        1 => None,
        n => Some(errors::too_many_progress_bars(&input, n)),
    }
}

fn get_labled_fields(fields: &Punctuated<Field, Comma>) -> Vec<&Field> {
    fields
        .iter()
        .filter(|f| {
            f.attrs
                .iter()
                .any(|a| a.path().is_ident(LABEL_NAME)) // can have more than one label 
        })
        .collect()
}

fn check_labled_field_count(input: &DeriveInput, len: usize) -> Option<TokenStream> {
    match len {
        0 => Some(errors::no_fields(&input)),
        1 => None,
        n => Some(errors::too_many_fields(&input, n)),
    }
}

fn get_labeled_value(labeled_field: &Field) -> Option<syn::Expr> {
    labeled_field
        .attrs
        .iter()
        .find_map(|attr| {
            if !attr.path().is_ident(LABEL_NAME) {
                return None;
            }
            let Meta::NameValue(nv) = &attr.meta else {
                return None;
            };
            Some(nv.value.clone())
        })
}

#[proc_macro_derive(ProgressBar, attributes(max_value))]
pub fn derive_progress_bar(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(f) => &f.named,
            _ => return errors::only_named_structs(&input),
        },
        _ => return errors::only_structs(&input),
    };

    let progress_bar_fields = get_progress_bar_fields(fields);

    if let Some(err) = check_progress_bar_field_count(&input, progress_bar_fields.len()) {
        return err;
    }

    let labeled = get_labled_fields(&fields);

    if let Some(err) = check_labled_field_count(&input, labeled.len()) {
        return err;
    }

    let progress_bar_field = progress_bar_fields[0];
    let progress_bar_ident = progress_bar_field
        .ident
        .as_ref()
        .unwrap();

    let labeled_field = labeled[0];
    let labeled_ident = labeled_field.ident.as_ref().unwrap();

    let label_value = match get_labeled_value(labeled_field) {
        Some(v) => v,
        None => return errors::parse_error(&input),
    };

    quote! {
        impl #struct_name {
            pub fn get_the_floaaaat(&self) -> f64 {
                self.#labeled_ident as f64 / #label_value as f64
            }
        }
        impl OutputBar for #struct_name {
            fn get_bar(&mut self) -> &mut ProgressBar {
                &mut self.#progress_bar_ident
            }
        }
    }
    .into()
}
