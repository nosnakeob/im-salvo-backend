extern crate proc_macro;

use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use regex::{Captures, Regex};
use syn::{ItemFn, parse_macro_input};
use syn::parse_quote;

#[proc_macro_attribute]
pub fn rb_conn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(item as ItemFn); // 我们传入的是一个函数，所以要用到ItemFn

    let func_decl = &mut func.sig; // 函数申明
    let func_inputs = &mut func_decl.inputs; // 函数输入参数
    let func_block = &mut func.block;

    // eprintln!("--------------------------");
    // eprintln!("func_inputs: {:?}", func_inputs);
    // 加输入参数引入登录请求守护
    func_inputs.push(parse_quote!(rb: &rocket::State<rbatis::RBatis>));
    // eprintln!("func_inputs: {:?}", func_inputs);

    func_block.stmts.insert(0, parse_quote! {
        let rb = &**rb;
    });

    let re = Regex::new(r"(\w+\s*::\s*(select|insert)\w*\()").unwrap();
    let func_block = func_block.to_token_stream().to_string();
    // eprintln!("{}", func_block);
    let new_func_block = re.replace_all(&func_block, |caps: &Captures| {
        format!("{}rb,", &caps[1])
    });
    // eprintln!("{}", new_func_block);

    func.block = syn::parse_str(&new_func_block).unwrap();

    // 重新构建函数执行
    let new_fn = quote!( #func );

    // eprintln!("new_fn: {}", new_fn);

    new_fn.into()
}

