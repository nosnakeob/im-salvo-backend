#![feature(proc_macro_span)]
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::{Span, TokenStream};

use syn::{ItemFn, LitStr};
use syn::parse_quote;

use crate::connection::{_rb_conn, _transaction};
use crate::route::{_auto_mount, _rocket_base_path};

mod visitor;
mod utils;
mod connection;
mod route;


#[proc_macro_attribute]
pub fn has_permit(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn); // 我们传入的是一个函数，所以要用到ItemFn
    let func_vis = &func.vis; // pub
    let func_block = &func.block;//

    let func_decl = &func.sig; // 函数申明
    let func_name = &func_decl.ident; // 函数名
    let func_asyncness = &func_decl.asyncness; // 函数名
    let func_generics = &func_decl.generics; // 函数泛型
    let func_inputs = &func_decl.inputs; // 函数输入参数
    let func_output = &func_decl.output; // 函数返回

    let permit = attr.to_string();

    quote!( // 重新构建函数执行
        #func_vis #func_asyncness fn #func_name #func_generics(#func_inputs) #func_output{
            #func_block
            // match crate::token_auth::check_permit(req_in_permit, #s).await {//fixme 判断参数中是否存在httpRequest，以后再说
            //      None =>  #func_block
            //  Some(res) => { return res.resp_json(); }
            // }
            println!(#permit);
        }
    ).into()
}

#[proc_macro_attribute]
pub fn loggedin(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(item as ItemFn); // 我们传入的是一个函数，所以要用到ItemFn

    let func_decl = &mut func.sig; // 函数申明
    let func_inputs = &mut func_decl.inputs; // 函数输入参数

    // 加输入参数引入登录请求守护
    func_inputs.push(parse_quote!(_user_claim: UserClaim));

    // 重新构建函数执行
    let new_fn = quote!( #func );

    // eprintln!("new_fn: {}", new_fn);

    new_fn.into()
}

#[proc_macro_attribute]
pub fn rb_conn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(item as ItemFn);

    _rb_conn(&mut func);

    let new_fn = quote!( #func );

    // eprintln!("new_fn: {}", new_fn);

    new_fn.into()
}

#[proc_macro_attribute]
pub fn transaction(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(item as ItemFn);

    _transaction(&mut func);

    let new_fn = quote!( #func );

    // eprintln!("new_fn: {}", new_fn);

    new_fn.into()
}

#[proc_macro]
pub fn rocket_base_path(input: TokenStream) -> TokenStream {
    let base_path = parse_macro_input!(input as LitStr);
    // eprintln!("input: {:?}", base_path);

    let new_fn = _rocket_base_path(base_path, Span::call_site().source_file().path());

    // eprintln!("{}", new_fn);

    new_fn
}

#[proc_macro_attribute]
pub fn auto_mount(attr: TokenStream, item: TokenStream) -> TokenStream {
    let dir = parse_macro_input!(attr as LitStr).value();
    // eprintln!("dir: {}", dir);
    let mut func = parse_macro_input!(item as ItemFn);

    _auto_mount(dir, &mut func);

    let new_fn = quote!( #func );

    // eprintln!("{}", new_fn);

    new_fn.into()
}
