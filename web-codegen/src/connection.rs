use syn::{ItemFn, Stmt};
use syn::visit_mut::VisitMut;

use crate::visitor::sql::{RbatisConn, Transaction};

pub fn _rb_conn(func: &mut ItemFn) {
    func.sig.inputs.push(parse_quote!(rb: &rocket::State<rbatis::RBatis>));

    RbatisConn.visit_item_fn_mut(func);

    func.block.stmts.insert(0, parse_quote! {
        let rb = &**rb;
    });
}

pub fn _transaction(func: &mut ItemFn) {
    func.sig.inputs.push(parse_quote!(rb: &rocket::State<rbatis::RBatis>));

    Transaction.visit_item_fn_mut(func);

    let v: Vec<Stmt> = parse_quote! {
        let rb = &**rb;
        let tx = &mut rb.acquire_begin().await.unwrap();
    };
    func.block.stmts.splice(0..0, v);

    let len = func.block.stmts.len();

    func.block.stmts.insert(len - 1, parse_quote! {
        tx.commit().await.unwrap();
    });
}
