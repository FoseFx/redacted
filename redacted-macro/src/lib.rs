extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Redact, attributes(redacted))]
pub fn derive_redact(item_stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item_stream as DeriveInput);

    let name = input.ident;
    let name_str = name.to_string();
    let data = input.data;

    let all_fields = find_fields(data);

    let fields = all_fields.into_iter().map(|(ident, redact)| {
        let ident_str = ident.to_string();
        let value = match redact {
            true => quote! { &"<redacted>" },
            false => quote! { &self.#ident },
        };

        return quote! {
            .field(#ident_str, #value)
        };
    });

    // This will be added after the struct
    let tree = quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
              f.debug_struct(#name_str)
                #(#fields)*
                .finish()
            }
        }
        impl Redact for #name {}
    };

    TokenStream::from(tree)
}

fn find_fields(data: Data) -> Vec<(proc_macro2::Ident, bool)> {
    let mut all_fields = vec![];

    if let Data::Struct(d_struct) = data {
        if let Fields::Named(fields) = d_struct.fields {
            for field in fields.named {
                if field.ident.is_none() {
                    continue;
                }
                let mut redact = false;
                for attr in field.attrs {
                    if attr.path.is_ident("redacted") {
                        redact = true;
                        break;
                    }
                }
                all_fields.push((field.ident.unwrap(), redact));
            }
        }
    }

    all_fields
}
