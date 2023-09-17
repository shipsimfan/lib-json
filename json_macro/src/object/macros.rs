macro_rules! ident_eq {
    ($token: expr, $ident: expr) => {
        match $token {
            proc_macro::TokenTree::Ident(ident) => ident.to_string() == $ident,
            _ => false,
        }
    };
}

pub(super) use ident_eq;
