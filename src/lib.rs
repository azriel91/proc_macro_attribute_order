extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_roids::FieldsUnnamedAppend;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, FieldsUnnamed, Ident};

macro_rules! make_attribute {
    ($name:ident) => {
        #[proc_macro_attribute]
        pub fn $name(args: TokenStream, item: TokenStream) -> TokenStream {
            let mut ast = parse_macro_input!(item as DeriveInput);
            let field_type = parse_macro_input!(args as Ident);

            let fields_additional: FieldsUnnamed = parse_quote!((#field_type,));
            ast.append_unnamed(fields_additional);

            let token_stream2 = quote! {
                #ast
            };

            token_stream2.into()
        }
    };
}

make_attribute!(attr_0);
make_attribute!(attr_1);
make_attribute!(attr_2);
