use super::*;

pub(crate) fn to_ident(name: &str) -> Ident {
    Ident::new(name, proc_macro2::Span::call_site())
}
