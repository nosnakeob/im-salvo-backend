#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

use syn::{ExprCall, ExprPath, ItemFn};
use syn::Expr::Path;
use syn::visit_mut::VisitMut;

struct RbatisConn;

impl VisitMut for RbatisConn {
    // xxx::xxx
    fn visit_expr_call_mut(&mut self, call: &mut ExprCall) {
        if let Path(ExprPath { ref path, .. }) = *call.func {
            if let Some(ident) = path.segments.last() {
                let ident_str = ident.ident.to_string();
                if ["select", "insert", "update"].iter().any(|&x| ident_str.starts_with(x)) {
                    call.args.insert(0, parse_quote!(rb));
                }
            }
        }

        // eprintln!("visit_expr_call_mut: {:?}", call.to_token_stream().to_string());
    }

    // xxx.xxx
    // fn visit_expr_method_call_mut(&mut self, call: &mut ExprMethodCall) {
    //     eprintln!("visit_expr_method_call_mut: {:?}", call.to_token_stream().to_string());
    // }
}

#[proc_macro_attribute]
pub fn rb_conn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(item as ItemFn);

    func.sig.inputs.push(parse_quote!(rb: &rocket::State<rbatis::RBatis>));

    RbatisConn.visit_item_fn_mut(&mut func);

    func.block.stmts.insert(0, parse_quote! {
        let rb = &**rb;
    });

    // 重新构建函数执行
    let new_fn = quote!( #func );

    // eprintln!("new_fn: {}", new_fn);

    new_fn.into()
}

