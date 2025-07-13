use crate::visitor::sql::RbatisConn;
use syn::ItemFn;
use syn::visit_mut::VisitMut;

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
