#![feature(proc_macro_span)]
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use crate::connection::_rb_conn;
use crate::route::{_auto_mount, _rocket_base_path};
use proc_macro::{Span, TokenStream};
use syn::parse_quote;
use syn::{ItemFn, LitStr};

mod connection;
mod route;
mod utils;
mod visitor;

#[proc_macro]
pub fn bail(input: TokenStream) -> TokenStream {
    let msg = parse_macro_input!(input as LitStr);
    let new_fn = quote! {
        return ApiResponse::from_error_msg(
            salvo::prelude::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            #msg
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

// #[proc_macro]
// pub fn rocket_base_path(input: TokenStream) -> TokenStream {
//     let base_path = parse_macro_input!(input as LitStr);
//
//     let new_fn = _rocket_base_path(base_path, Span::call_site().source_file().path());
//
//     // eprintln!("{}", new_fn);
//
//     new_fn
// }

// 约定扫描当前调用宏的文件目录下的controller目录
// #[proc_macro_attribute]
// pub fn auto_mount(_attr: TokenStream, item: TokenStream) -> TokenStream {
//     let mut dir = Span::call_site().source_file().path();
//     dir.pop();
//     dir.push("controller");
//     let mut func = parse_macro_input!(item as ItemFn);
//
//     _auto_mount(&dir.to_string_lossy(), &mut func);
//
//     let new_fn = quote!( #func );
//
//     // eprintln!("{}", new_fn);
//
//     new_fn.into()
// }
