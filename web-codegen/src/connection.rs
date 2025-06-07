use syn::visit_mut::VisitMut;
use syn::{ItemFn, Stmt};

use crate::visitor::sql::{RbatisConn, Transaction};

pub fn _rb_conn(func: &mut ItemFn) {
    func.sig
        .inputs
        .push(parse_quote!(rb: &rocket::State<rbatis::RBatis>));

    RbatisConn.visit_item_fn_mut(func);

    func.block.stmts.insert(
        0,
        parse_quote! {
            let rb = &**rb;
        },
    );
}
