#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use crate::connection::_rb_conn;
use proc_macro::{TokenStream};
use syn::{parse_quote, Expr, Fields, ItemFn, ItemStruct, LitStr};

mod connection;
mod utils;
mod visitor;

#[proc_macro]
pub fn bail(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);
    let new_fn = quote! {
        return ApiResponse::from_error_msg(
            salvo::prelude::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            #expr
        );
    };
    // eprintln!("{}", new_fn);
    new_fn.into()
}

#[proc_macro_attribute]
pub fn rbatis_conn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(item as ItemFn);

    _rb_conn(&mut func);

    let new_fn = quote!( #func );

    // eprintln!("new_fn: {}", new_fn);

    new_fn.into()
}

#[proc_macro_attribute]
pub fn base_entity(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut class = parse_macro_input!(item as ItemStruct);

    if let Fields::Named(ref mut fields) = class.fields {
        // 添加 created_at 和 updated_at 字段
        fields.named.push(parse_quote! {
            pub created_at: Option<rbatis::rbdc::DateTime>
        });
        fields.named.push(parse_quote! {
            pub updated_at: Option<rbatis::rbdc::DateTime>
        });
    }

    let new_class = quote! (#class);

    // eprintln!("new_class: {}", new_class);

    new_class.into()
}
